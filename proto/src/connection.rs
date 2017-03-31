use std::collections::{VecDeque};
use std::io::Result;
use events;
use jid::Jid;
use config::XMPPConfig;
use base64::{encode};
use std::str;
use credentials::Credentials;

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

#[derive(Clone,Debug,PartialEq)]
pub struct Connection {
    pub state: ConnectionState,
    config: XMPPConfig,
    pub credentials: Option<Credentials>,
    /// list of message to send
    pub frame_queue:       VecDeque<String>,
    pub input_queue:       VecDeque<String>,
}

impl Connection {
    pub fn new(config: XMPPConfig) -> Connection {
        Connection {
            state: ConnectionState::Initial,
            credentials: None,
            config: config,
            frame_queue: VecDeque::new(),
            input_queue: VecDeque::new()
        }
    }

    pub fn connect(&mut self) -> Result<ConnectionState> {
        self.frame_queue.push_back(events::OpenStreamEvent::new(&self.config).compute());

        Ok(ConnectionState::Connecting(ConnectingState::SentInitialStreamHeader))
    }

    pub fn next_frame(&mut self) -> Option<String> {
        self.frame_queue.pop_front()
    }

    pub fn next_input_frame(&mut self) -> Option<String> {
        self.input_queue.pop_front()
    }

    pub fn add_input_frame(&mut self, f: String) {
        self.input_queue.push_back(f)
    }

    pub fn add_frame(&mut self, f: String) {
        self.frame_queue.push_back(f)
    }

    pub fn start_tls(&mut self) {
        let event = events::OpenStreamEvent::new(&self.config);
        self.frame_queue.push_back(event.compute());
    }

    pub fn compile_presence(&mut self) {
        match self.credentials {
            Some(ref c) => {
                let p = format!("<presence from='{}' />", c.jid);
                self.frame_queue.push_back(p);
            },
            None => {}
        };
    }

    pub fn handle_frame(&mut self, f: String) {
        if f.contains("result") {
            self.state = ConnectionState::Connected;
            let jid_split = f.split("<jid>").collect::<Vec<&str>>();
            let jid_split_next = jid_split[1].split("</jid>").collect::<Vec<&str>>();
            self.credentials = Some(Credentials {jid: Jid::from_full_jid(jid_split_next[0]), password: "tt".to_string()});
        }
        if f.contains("stream:features") {
            self.state = ConnectionState::Connecting(ConnectingState::ReceivedInitialStream);

            if f.contains("starttls") {
                self.state = ConnectionState::Connecting(ConnectingState::ReceivedInitialStreamFeatures);
                self.frame_queue.push_back(events::StartTlsEvent::new(&self.config).compute());
            }
            if f.contains("PLAIN") {
                self.state = ConnectionState::Connecting(ConnectingState::ReceivedAuthenticatedFeatures);
                let mut data: Vec<u8> = Vec::new();
                data.push(0);
                data.extend(b"alice@example.com");
                data.push(0);
                data.extend(b"test");

                let bytes = str::from_utf8(&data).unwrap().as_bytes();
                let plain = encode(bytes);
                let plain = format!("<auth xmlns='urn:ietf:params:xml:ns:xmpp-sasl' mechanism='PLAIN'>{}</auth>", plain);
                self.frame_queue.push_back(plain);
            }
            else if f.contains("session") {
                self.frame_queue.push_back("<iq type='set' id='bind_1'><bind xmlns='urn:ietf:params:xml:ns:xmpp-bind'/></iq>".to_string());
            }
        } else if f.contains("proceed") {
            self.state = ConnectionState::Connecting(ConnectingState::ReceivedProceedCommand);
            self.start_tls();
        } else if f.contains("success") {
            self.frame_queue.push_back("<stream:stream xmlns='jabber:client' xmlns:stream='http://etherx.jabber.org/streams' to='{}' version='1.0'>".to_string());

        } else if f.contains("bind_") {
            trace!("binded");
        } else if self.state == ConnectionState::Connected {
            self.add_input_frame(f);
        }

        // match f {
        //     Frame::ProtocolHeader => {
        //         // println!("error: the client should not receive a protocol header");
        //         self.state = ConnectionState::Error;
        //     },
        //     Frame::Method(channel_id, method) => {
        //         if channel_id == 0 {
        //             self.handle_global_method(method);
        //         } else {
        //             self.receive_method(channel_id, method);
        //         }
        //     },
        //     Frame::Heartbeat(_) => {
        //         self.frame_queue.push_back(Frame::Heartbeat(0));
        //     },
        //     Frame::Header(channel_id, _, header) => {
        //         self.handle_content_header_frame(channel_id, header.body_size);
        //     },
        //     Frame::Body(channel_id, payload) => {
        //         self.handle_body_frame(channel_id, payload);
        //     }
        // };
    }
}
