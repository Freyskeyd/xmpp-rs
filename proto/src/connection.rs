use std::collections::{VecDeque};
use std::io::Result;
use events;
use ns;
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
    credentials: Option<Credentials>,
    /// list of message to send
    pub frame_queue:       VecDeque<String>,
}

impl Connection {
    pub fn new(config: XMPPConfig) -> Connection {
        Connection {
            state: ConnectionState::Initial,
            credentials: None,
            config: config,
            frame_queue: VecDeque::new()
        }
    }

    pub fn connect(&mut self) -> Result<ConnectionState> {
        self.frame_queue.push_back(events::OpenStreamEvent::new(&self.config).compute());

        Ok(ConnectionState::Connecting(ConnectingState::SentInitialStreamHeader))
    }

    pub fn next_frame(&mut self) -> Option<String> {
        self.frame_queue.pop_front()
    }

    pub fn add_frame(&mut self, f: String) {
        self.frame_queue.push_back(f)
    }

    pub fn start_tls(&mut self) {
        let event = events::OpenStreamEvent::new(&self.config);
        self.frame_queue.push_back(event.compute());
    }

    pub fn handle_frame(&mut self, f: String) {
        if f.contains("result") {
            self.state = ConnectionState::Connected;
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

// #[cfg(test)]
// mod tests {
//     use std::net::TcpListener;
//     use tokio_io::{AsyncRead,AsyncWrite};
//     use tokio_io::codec::{Framed};
//     use tokio_core::reactor::Core;
//     use tokio_core::net::TcpStream;
//     use std::net::SocketAddr;
//     use std::io::{Read,Write};
//     use futures::Future;
//     use futures::future;
//     use std::thread;

//     macro_rules! t {
//         ($e:expr) => {
//             match $e {
//                 Ok(t) => t,
//                 Err(e) => panic!("received error for `{}`: {}", stringify!($e), e),
//             }
//         }
//     }

//     #[test]
//     fn listen_localhost() {
//         let socket_addr:SocketAddr = "127.0.0.1:5222".parse().unwrap();
//         let listener = t!(TcpListener::bind(&socket_addr));

//         let _t = thread::spawn(move || {
//             let mut core = Core::new().unwrap();
//             let handler = core.handle();

//             core.run(
//                 TcpStream::connect(&socket_addr, &handler)
//                 .and_then(|stream| XMPPTransport::connect(stream.framed(XMPPCodec)))
//                 .and_then(|transport| {
//                     Ok(())
//                 })
//                 ).unwrap();

//             // connection.connect()
//         });

//         let mut stream = t!(listener.accept()).0;
//         let mut buf = Vec::new();
//         t!(stream.read_to_end(&mut buf));
//         assert!(String::from_utf8(buf).unwrap() == "hello");

//         let result = _t.join();

//         assert!(!result.is_err());
//     }
// }
