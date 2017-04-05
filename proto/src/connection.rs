use std::collections::{VecDeque};
use std::io::Result;
use jid::Jid;
use config::XMPPConfig;
use std::str;
use credentials::Credentials;
use events::Event;
use events::*;
use std::str::FromStr;
use ns;

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
}

impl Connection {
    pub fn new(config: &XMPPConfig, credentials: Option<Credentials>) -> Connection {
        Connection {
            state: ConnectionState::Initial,
            credentials: credentials,
            config: config.clone(),
            frame_queue: VecDeque::new(),
            input_queue: VecDeque::new()
        }
    }

    pub fn connect(&mut self) -> Result<ConnectionState> {
        self.frame_queue.push_back(Event::NonStanza(NonStanzaEvent::OpenStream(OpenStream::new(&self.config)), String::new()));

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
        let event = Event::NonStanza(NonStanzaEvent::OpenStream(OpenStream::new(&self.config)), String::new());
        self.frame_queue.push_back(event);
    }

    pub fn compile_presence(&mut self) {
        if let Some(ref c) = self.credentials {
            let event = Event::Stanza(StanzaEvent::Presence(Presence::new(&self.config, c.jid.clone())), String::new());
            self.frame_queue.push_back(event);
        }
    }

    pub fn handle_frame(&mut self, f: Event) {
        match f {
            Event::NonStanza(NonStanzaEvent::ProceedTls(_), _) => {
                self.state = ConnectionState::Connecting(ConnectingState::ReceivedProceedCommand);
                self.start_tls();
            }
            Event::NonStanza(NonStanzaEvent::SuccessTls(_), _) => {
                self.frame_queue.push_back(Event::NonStanza(NonStanzaEvent::OpenStream(OpenStream::new(&self.config)), String::new()));
            },
            Event::NonStanza(NonStanzaEvent::StreamFeatures(_), source) => {
                if source.contains("starttls") {
                    self.state = ConnectionState::Connecting(ConnectingState::ReceivedInitialStreamFeatures);
                    self.frame_queue.push_back(Event::NonStanza(NonStanzaEvent::StartTls(StartTls::new(&self.config)), String::new()));
                } else if source.contains("PLAIN") {
                    self.state = ConnectionState::Connecting(ConnectingState::ReceivedAuthenticatedFeatures);
                    let credentials = match self.credentials {
                        Some(ref c) => c.clone(),
                        None => Credentials { ..Credentials::default() }
                    };
                    self.frame_queue.push_back(Event::NonStanza(NonStanzaEvent::Auth(Auth::new(&self.config, credentials)), String::new()));
                } else if source.contains("session") {
                    let bind = Bind::new()
                        .set_type("set")
                        .set_id("bind_1");

                    self.frame_queue.push_back(Event::Stanza(StanzaEvent::Iq(IqType::Bind(bind)), String::new()));
                } else {
                    self.state = ConnectionState::Connecting(ConnectingState::ReceivedInitialStream);
                }
            },
            Event::Stanza(StanzaEvent::IqResponse(IqType::Bind(event)), _) => {
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
            Event::Stanza(StanzaEvent::Iq(IqType::Generic(event)), source) => {
                if event.body.is_some() {
                    if event.body.unwrap().find((ns::BIND, "bind")).is_some() {
                        if event.iq_type == "result" {
                            self.handle_frame(Event::Stanza(StanzaEvent::IqResponse(IqType::Bind(Bind::from_str(&source).unwrap())), source));
                        }
                    }
                }
            },
            _ => {
                if self.state == ConnectionState::Connected {
                    self.add_input_frame(f);
                }
            }
        }
    }
}
