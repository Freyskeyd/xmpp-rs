use std::collections::{HashMap,VecDeque};
use std::io::Result;
use jid::Jid;
use config::XMPPConfig;
use credentials::Credentials;
use events::Event;
use events::Event::*;
use events::NonStanzaEvent::*;
use events::StanzaEvent::*;
use events::IqEvent::*;
use events::*;
use events::FromGeneric;
use ns;
use futures::sync::oneshot::Sender;

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

    pub fn connect(&mut self) -> Result<ConnectionState> {
        self.frame_queue.push_back(NonStanza(Box::new(OpenStreamEvent(OpenStream::new(&self.config))), String::new()));

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
        let event = NonStanza(Box::new(OpenStreamEvent(OpenStream::new(&self.config))), String::new());
        self.frame_queue.push_back(event);
    }

    pub fn compile_ping(&mut self) -> Ping {
        if let Some(ref c) = self.credentials {
            return Ping::new(&c.jid, self.config.get_domain());
        }
        panic!("")
    }

    pub fn compile_presence(&mut self) {
        if let Some(_) = self.credentials {
            let presence = Presence::new();
            self.frame_queue.push_back(presence.to_event());
        }
    }

    pub fn handle_frame(&mut self, f: Event) {
        let returnable_event = f.clone();
        match f {
            NonStanza(non_stanza, source) => {
                match *non_stanza {
                    CloseStreamEvent => {
                        self.state = ConnectionState::Closed;
                        self.frame_queue.push_back(returnable_event);
                    }
                    ProceedTlsEvent(_) => {
                        self.state = ConnectionState::Connecting(ConnectingState::ReceivedProceedCommand);
                        self.start_tls();
                    }
                    SuccessTlsEvent(_) => {
                        self.frame_queue.push_back(NonStanza(Box::new(OpenStreamEvent(OpenStream::new(&self.config))), String::new()));
                    },
                    StreamFeaturesEvent(_) => {
                        if source.contains("starttls") {
                            self.state = ConnectionState::Connecting(ConnectingState::ReceivedInitialStreamFeatures);
                            self.frame_queue.push_back(NonStanza(Box::new(StartTlsEvent(StartTls::new(&self.config))), String::new()));
                        } else if source.contains("PLAIN") {
                            self.state = ConnectionState::Connecting(ConnectingState::ReceivedAuthenticatedFeatures);
                            let credentials = match self.credentials {
                                Some(ref c) => c.clone(),
                                None => Credentials { ..Credentials::default() }
                            };
                            let auth = Auth::new(&self.config, credentials);

                            self.frame_queue.push_back(auth.to_event());
                        } else if source.contains("session") {
                            let mut bind = Bind::new();
                            bind.set_type(IqType::Set)
                                .unwrap()
                                .set_id("bind_1");

                            self.frame_queue.push_back(bind.to_event());
                        } else {
                            self.state = ConnectionState::Connecting(ConnectingState::ReceivedInitialStream);
                        }
                    },
                    e => {
                        trace!("{:?}", e);
                        if self.state == ConnectionState::Connected {
                            self.add_input_frame(returnable_event);
                        }
                    }
                }
            },
            Stanza(stanza, _) => {
                match *stanza {
                    IqRequestEvent(iq) => {
                        match *iq {
                            _ => {
                                if self.state == ConnectionState::Connected {
                                    self.add_input_frame(returnable_event);
                                }
                            }
                        }
                    },
                    IqEvent(iq) => {
                        match *iq {
                            GenericEvent(event) => {
                                if event.get_type() == IqType::Result {
                                    match event.get_element() {
                                        Some(body) if body.find((ns::BIND, "bind")).is_some() => {
                                            let bind = Bind::from_generic(&event).unwrap();
                                            self.handle_frame(bind.to_event());
                                        },
                                        Some(_) => self.handle_iq(event.get_id(), returnable_event),
                                        None => {}
                                    }
                                }
                            },
                            _ => {
                                if self.state == ConnectionState::Connected {
                                    self.add_input_frame(returnable_event);
                                }
                            }
                        }

                    },
                    IqResponseEvent(iq) => {
                        match *iq {
                            GenericEvent(event) => {
                                if event.get_type() == IqType::Result {
                                    match event.get_element() {
                                        Some(body) if body.find((ns::BIND, "bind")).is_some() => {
                                            let bind = Bind::from_generic(&event).unwrap();
                                            self.handle_frame(bind.to_event());
                                        },
                                        Some(_) => self.handle_iq(event.get_id(), returnable_event),
                                        None => {}
                                    }
                                }
                            },
                            BindEvent(event) => {
                                if let Some(body) = event.generic.get_element() {
                                    if let Some(bind) = body.find((ns::BIND, "bind")) {
                                        if let Some(jid) = bind.find((ns::BIND, "jid")) {
                                            self.state = ConnectionState::Connected;
                                            if let Some(ref mut c) = self.credentials {
                                                c.jid = Jid::from_full_jid(jid.text())
                                            }
                                        }
                                    }
                                }
                            },
                            _ => {
                                if self.state == ConnectionState::Connected {
                                    self.add_input_frame(returnable_event);
                                }
                            }
                        }
                    },
                    _ => {
                        if self.state == ConnectionState::Connected {
                            self.add_input_frame(returnable_event);
                        }
                    }
                }
            },
            _ => {
                if self.state == ConnectionState::Connected {
                    self.add_input_frame(returnable_event);
                }
            }
        }
    }

    fn handle_iq(&mut self, id: &str, event: Event) {
        if let Some(tx) = self.iq_queue.remove(id) {
                let _ = tx.send(event);
        } else {
            self.add_input_frame(event);
        }
    }
}
