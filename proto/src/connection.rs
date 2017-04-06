use std::collections::{HashMap,VecDeque};
use std::io::Result;
use jid::Jid;
use config::XMPPConfig;
use credentials::Credentials;
use events::Event;
use events::Event::*;
use events::NonStanzaEvent::*;
use events::StanzaEvent::*;
use events::IqType::*;
use events::*;
use std::str::FromStr;
use ns;
use std::sync::Arc;
use futures::sync::oneshot::Sender;
use std::sync::Mutex;

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum ConnectionState {
    Initial,
    Connecting(ConnectingState),
    Connected,
    Closed,
    Error,
}

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum ConnectingState {
    Initial,

    SentInitialStreamHeader,
    ReceivedInitialStream,
    ReceivedInitialStreamFeatures,
    SentStarttlsCommand,
    ReceivedProceedCommand,
    ReceivedFailureTls,
    TlsNegociationSuccess,
    TlsNegociationFailed,

    SentTlsStreamHeader,
    ReceivedTlsStream,
    ReceivedStreamFeatures,

    SentAuthenticationMechanism,
    ReceivedAuthenticationMechanismError,

    ReceivedSuccessfulAuthentification,
    ReceivedFailureAuthentification,

    SentAuthenticatedStream,
    ReceivedAuthenticatedFeatures,

    Error,
}

#[derive(Debug)]
pub struct Connection {
    pub state: ConnectionState,
    config: XMPPConfig,
    pub credentials: Option<Credentials>,
    /// list of message to send
    pub frame_queue:       VecDeque<Event>,
    pub input_queue:       VecDeque<Event>,
    pub iq_queue: HashMap<String, Box<Sender<Event>>>,
}

impl Connection {
    pub fn new(config: &XMPPConfig, credentials: Option<Credentials>) -> Connection {
        Connection {
            state: ConnectionState::Initial,
            credentials: credentials,
            config: config.clone(),
            frame_queue: VecDeque::new(),
            input_queue: VecDeque::new(),
            iq_queue: HashMap::new()
        }
    }

    // pub fn fetch_iq_response(&mut self, id: &str) -> Event {
    //     let event = match self.iq_queue.get(id) {
    //         Some(e) => e.clone().unwrap(),
    //         None => panic!("")
    //     };
    //     self.iq_queue.remove(id);
    //     event
    // }
    // pub fn is_finished(&mut self, id: &str) -> bool {
    //     match self.iq_queue.get(id) {
    //         Some(e) => match *e {
    //             Some(ref e) => true,
    //             None => false
    //         },
    //         None => false
    //     }
    // }
    pub fn connect(&mut self) -> Result<ConnectionState> {
        self.frame_queue.push_back(NonStanza(OpenStreamEvent(OpenStream::new(&self.config)), String::new()));

        Ok(ConnectionState::Connecting(ConnectingState::SentInitialStreamHeader))
    }

    pub fn next_frame(&mut self) -> Option<Event> {
        self.frame_queue.pop_front()
    }

    pub fn next_input_frame(&mut self) -> Option<Event> {
        self.input_queue.pop_front()
    }

    pub fn add_input_frame(&mut self, f: Event) {
        self.input_queue.push_back(f)
    }

    pub fn add_frame(&mut self, f: Event) {
        self.frame_queue.push_back(f)
    }

    pub fn start_tls(&mut self) {
        let event = NonStanza(OpenStreamEvent(OpenStream::new(&self.config)), String::new());
        self.frame_queue.push_back(event);
    }

    pub fn compile_ping(&mut self) -> Ping {
        if let Some(ref c) = self.credentials {

            return Ping::new(c.jid.clone(), self.config.get_domain());
        }
        panic!("")
    }

    pub fn compile_presence(&mut self) {
        if let Some(ref c) = self.credentials {
            let event = Stanza(PresenceEvent(Presence::new(&self.config, c.jid.clone())), String::new());
            self.frame_queue.push_back(event);
        }
    }

    pub fn handle_frame(&mut self, f: Event) {
        match f {
            NonStanza(ProceedTlsEvent(_), _) => {
                self.state = ConnectionState::Connecting(ConnectingState::ReceivedProceedCommand);
                self.start_tls();
            }
            NonStanza(SuccessTlsEvent(_), _) => {
                self.frame_queue.push_back(NonStanza(OpenStreamEvent(OpenStream::new(&self.config)), String::new()));
            },
            NonStanza(StreamFeaturesEvent(_), source) => {
                if source.contains("starttls") {
                    self.state = ConnectionState::Connecting(ConnectingState::ReceivedInitialStreamFeatures);
                    self.frame_queue.push_back(NonStanza(StartTlsEvent(StartTls::new(&self.config)), String::new()));
                } else if source.contains("PLAIN") {
                    self.state = ConnectionState::Connecting(ConnectingState::ReceivedAuthenticatedFeatures);
                    let credentials = match self.credentials {
                        Some(ref c) => c.clone(),
                        None => Credentials { ..Credentials::default() }
                    };
                    self.frame_queue.push_back(NonStanza(AuthEvent(Auth::new(&self.config, credentials)), String::new()));
                } else if source.contains("session") {
                    let bind = Bind::new()
                        .set_type("set")
                        .set_id("bind_1");

                    self.frame_queue.push_back(Stanza(IqEvent(BindIq(bind)), String::new()));
                } else {
                    self.state = ConnectionState::Connecting(ConnectingState::ReceivedInitialStream);
                }
            },
            Stanza(IqResponseEvent(BindIq(event)), _) => {
                match event.body {
                    Some(body) => match body.find((ns::BIND, "bind")) {
                        Some(bind) => match bind.find((ns::BIND, "jid")) {
                            Some(jid) => {
                                self.state = ConnectionState::Connected;
                                match self.credentials {
                                    Some(ref mut c) => c.jid = Jid::from_full_jid(&jid.text()),
                                    None => {}
                                };
                            },
                            None => {}
                        },
                        None => {}
                    },
                    None => {}
                }
            },
            Stanza(IqEvent(GenericIq(event)), source) => {
                let e = event.clone();
                if event.body.is_some() {
                    if event.body.unwrap().find((ns::BIND, "bind")).is_some() {
                        if event.iq_type == "result" {
                            self.handle_frame(Stanza(IqResponseEvent(BindIq(Bind::from_str(&source).unwrap())), source));
                        }
                    } else if event.iq_type == "result" {
                        self.handle_iq(event.id.to_string(), Stanza(IqEvent(GenericIq(e.clone())), String::new()));
                        // self.handle_frame(Stanza(IqEvent(GenericIq(event)), source));
                    }
                }
            },
            // Stanza(IqResponseEvent(iq), _) => {
            //     println!("hey");
            //     match iq {
            //         PingIq(ping) => {
            //             if self.iq_queue.contains_key(&ping.id) {
            //                 self.iq_queue.insert(ping.id.to_string(), Some(PingIq(ping)));
            //             }
            //         },
            //         _ => {},
            //     }
            // },
            _ => {
                if self.state == ConnectionState::Connected {
                    self.add_input_frame(f);
                }
            }
        }
    }

    fn handle_iq(&mut self, id: String, event: Event) {
        match self.iq_queue.remove(&id) {
            Some(tx) => {
                tx.send(event);
            },
            None => {}
        }
    }
}
