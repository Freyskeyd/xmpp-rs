extern crate xmpp_proto;
extern crate base64;
extern crate futures;
extern crate tokio_core;
extern crate native_tls;
extern crate tokio_tls;
extern crate tokio_io;
extern crate bytes;

use base64::{encode};
use futures::{Future, Stream, Sink};
use native_tls::TlsConnector;
use std::str;
use std::{io};
use tokio_core::net::TcpStream as TokioStream;
use std::net::TcpStream;
use tokio_core::reactor::Core;
use tokio_io::{AsyncRead};
use tokio_tls::TlsConnectorExt;
use tokio_io::codec::Framed;
use futures::sync::mpsc;
use tokio_tls::TlsStream;
use std::io::prelude::*;

use std::thread;
use tokio_core::reactor::Handle;
use futures::sink::Send;
use futures::AndThen;
use futures::Map;
use futures::future::BoxFuture;
use std::io::Error;
use std::sync::Arc;

use xmpp_proto::codec::{ClientToServerCodec, LineCodec};
use xmpp_proto::connect::{ClientMessage, ServerMessage};
use xmpp_proto::stanza;

pub struct StanzaConfig {
    domain: String,
    xmpp_domain: String
}

impl StanzaConfig {
    pub fn get_xmpp_domain(&self) -> String {
        self.xmpp_domain.clone()
    }
    pub fn get_domain(&self) -> String {
        self.domain.clone()
    }
}
pub struct Connection {
    transport: Option<Framed<TokioStream, LineCodec>>
}

impl Connection {
    pub fn new() -> Connection {
        Connection {transport: None}
    }

    pub fn connect(&mut self, domain: &str, handler: &Handle) {
        let stream = TcpStream::connect((domain, 5222)).unwrap();
        let socket = TokioStream::from_stream(stream, handler).unwrap();
        let transport = socket.framed(LineCodec);
        self.transport = Some(transport);
    }

    pub fn authenticate(self, stanza_config: Arc<StanzaConfig>) 
        -> BoxFuture<(Option<String>, Framed<TlsStream<TokioStream>, LineCodec>), Error>
        {
        let start = format!("<?xml version='1.0'?><stream:stream version='1.0' xmlns:stream='http://etherx.jabber.org/streams' to='{}' xmlns='jabber:client'>", stanza_config.get_xmpp_domain());

        let auth_cfg = stanza_config.clone();
        let open_tls_stream = move |socket: TlsStream<TokioStream>| {
            let transport = socket.framed(LineCodec);

            let tls_success = format!("<stream:stream xmlns='jabber:client' xmlns:stream='http://etherx.jabber.org/streams' to='{}' version='1.0'>", auth_cfg.get_xmpp_domain());
            transport.send(tls_success.to_string())
                .and_then(|transport| transport.into_future().map_err(|(e, _)| e))
        };

        // Open xmpp stream
        let init_stream = self.transport.unwrap().send(start)
            .and_then(|transport| transport.into_future().map_err(|(e, _)| e));

        // Select Authentification
        let send_mechanism = init_stream
            .and_then(|(response, transport)| {
                println!("Response feature: {:?}", response);
                transport.send(stanza::non_stanza::AUTH.to_string())
            })
            .and_then(|transport| transport.into_future().map_err(|(e, _)| e));

        // Transform to TLS
        let tls = send_mechanism.and_then(move |(response, transport)| {
                println!("Response STARTTLS: {:?}", response);
                let builder = TlsConnector::builder().unwrap();
                let cx = builder.build().unwrap();

                println!("connected");
                cx.connect_async(stanza_config.get_domain().as_str(), transport.into_inner()).map_err(|e| {
                    io::Error::new(io::ErrorKind::Other, e)
                })
            });

        // Opening new TLS stream and wait for feature
        tls.and_then(open_tls_stream)
            .boxed()
    }
}

pub struct Client {
    connection: Connection,
    connected: bool,
    username: String,
    password: String,
    domain: String,
    xmpp_domain: String
}

impl Client {
    pub fn new<S>(username: S, password: S, domain: S, xmpp_domain: S) -> Client
        where S: ToString + 'static
    {
        Client {
            connection: Connection::new(),
            connected: false,
            username: username.to_string(),
            password: password.to_string(),
            domain: domain.to_string(),
            xmpp_domain: xmpp_domain.to_string()
        }
    }

    pub fn connect(mut self,out_tx: mpsc::Sender<String>) {
        thread::spawn(move || {
            let mut core = Core::new().unwrap();
            let (tx, rx) = mpsc::channel(1);

            self.connection.connect(&self.domain.as_str(), &core.handle());

            let cfg = Arc::new(StanzaConfig { domain: String::from(self.domain), xmpp_domain: String::from(self.xmpp_domain) });
            let start_stream = self.connection.authenticate(cfg.clone());

            let send_to_server = |msg| {
                match tx.clone().start_send(msg) {
                    Ok(_) => println!("message sent"),
                    Err(_) => println!("fail to send to sink")
                }
            };

            let xmpp_domain = cfg.get_xmpp_domain();

            let socket = start_stream
                .and_then(|(_, transport)| {
                    let mut data: Vec<u8> = Vec::new();
                    data.push(0);
                    data.extend(b"alice@example.com");
                    data.push(0);
                    data.extend(b"test");

                    let bytes = str::from_utf8(&data).unwrap().as_bytes();
                    let plain = encode(bytes);
                    let plain = format!("{}{}</auth>", stanza::non_stanza::PLAIN, plain);
                    transport.send(plain)
                })
                .and_then(|transport| transport.into_future().map_err(|(e, _)| e))
                .and_then(|(_, transport)| {
                    transport.send(format!("<stream:stream xmlns='jabber:client' xmlns:stream='http://etherx.jabber.org/streams' to='{}' version='1.0'>", xmpp_domain))
                })

                .and_then(|transport| transport.into_future().map_err(|(e, _)| e))
                .and_then(|(_, transport)| {
                    transport.send("<iq type='set' id='bind_1'><bind xmlns='urn:ietf:params:xml:ns:xmpp-bind'/></iq>".to_string())
                })
                .and_then(|transport| transport.into_future().map_err(|(e, _)| e))
                .and_then(|(response, transport)| {
                    println!("{:?}", response);
                    let socket = transport.into_inner();
                    let transport = socket.framed(ClientToServerCodec::new());

                    let (to_server, from_server) = transport.split();
                    let reader = from_server
                        .for_each(move |msg| {

                            Ok(())
                        });

                    let writer = rx
                        .map_err(|()| unreachable!("rx can't fail"))
                        .fold(to_server, |to_server, msg| {
                            to_server.send(msg)
                        })
                    .map(|_| ());

                    let _ = out_tx.clone().start_send("connected".to_string());
                    reader.select(writer).map(|_| ()).map_err(|(err, _)| err)

                });

            core.run(socket).unwrap();
        });
    }
}

// pub fn connect_client<F>(out_tx: mpsc::Sender<(ClientMessage, mpsc::Sender<ClientMessage>)>, f: F) 
//     where F: Fn(ServerMessage) -> Option<ClientMessage> + 'static
// {
//     let (tx, rx) = mpsc::channel(1);
//     let send_to_server = |msg| {
//         match tx.clone().start_send(msg) {
//             Ok(_) => println!("message sent"),
//             Err(_) => println!("fail to send to sink")
//         }
//     };

//     let domain = "xmpp-qa.iadvize.com";
//     let xmpp_domain = "bot.simon.iadvize.com";
//     let start = format!("<?xml version='1.0'?><stream:stream version='1.0' xmlns:stream='http://etherx.jabber.org/streams' to='{}' xmlns='jabber:client'>", xmpp_domain);
//     let tls_success = format!("<stream:stream xmlns='jabber:client' xmlns:stream='http://etherx.jabber.org/streams' to='{}' version='1.0'>", xmpp_domain);


//     let mut core = Core::new().unwrap();

//     let stream = TcpStream::connect((domain, 5222)).unwrap();
//     let socket = TokioStream::from_stream(stream, &core.handle()).unwrap();

//     let transport = socket.framed(LineCodec);

//     let starttls = |(response, t): (Option<String>, Framed<TokioStream, LineCodec>)| {
//         println!("Response START: {:?}", response);
//         t.send(stanza::non_stanza::AUTH.to_string())
//             .and_then(|transport| transport.into_future().map_err(|(e, _)| e))
//     };

//     let negociate = |(response, transport): (Option<String>, Framed<TokioStream, LineCodec>)| {
//         println!("Response STARTTLS: {:?}", response);
//         let builder = TlsConnector::builder().unwrap();
//         let cx = builder.build().unwrap();

//         println!("connected");
//         cx.connect_async(domain, transport.into_inner()).map_err(|e| {
//             io::Error::new(io::ErrorKind::Other, e)
//         })
//     };

//     let open_tls_stream = |socket: TlsStream<TokioStream>| {
//         let transport = socket.framed(LineCodec);

//         transport.send(tls_success.to_string())
//             .and_then(|transport| transport.into_future().map_err(|(e, _)| e))
//     };

//     let socket = transport.send(start)
//         .and_then(|transport| transport.into_future().map_err(|(e, _)| e))
//         .and_then(starttls)
//         .and_then(negociate)
//         .and_then(open_tls_stream)
//         .and_then(|(_, transport)| {
//             let mut data: Vec<u8> = Vec::new();
//             data.push(0);
//             data.extend(b"alice@example.com");
//             data.push(0);
//             data.extend(b"test");

//             // let plain = data.to_base64();

//             let bytes = str::from_utf8(&data).unwrap().as_bytes();
//             let plain = encode(bytes);
//             let plain = format!("{}{}</auth>", stanza::non_stanza::PLAIN, plain);
//             transport.send(plain)
//         })
//     .and_then(|transport| transport.into_future().map_err(|(e, _)| e))
//         .and_then(|(_, transport)| {
//             transport.send(format!("<stream:stream xmlns='jabber:client' xmlns:stream='http://etherx.jabber.org/streams' to='{}' version='1.0'>", xmpp_domain))
//         })

//     .and_then(|transport| transport.into_future().map_err(|(e, _)| e))
//     .and_then(|(_, transport)| {
//         transport.send("<iq type='set' id='bind_1'><bind xmlns='urn:ietf:params:xml:ns:xmpp-bind'/></iq>".to_string())
//         })
//     .and_then(|transport| transport.into_future().map_err(|(e, _)| e))
//     .and_then(|(response, transport)| {
//         println!("{:?}", response);
//         let socket = transport.into_inner();
//         let transport = socket.framed(ClientToServerCodec::new());

//         let (to_server, from_server) = transport.split();
//         let reader = from_server
//             .for_each(move |msg| {
//                 match f(msg) {
//                     Some(ret) => send_to_server(ret),
//                     None => {}
//                 };

//                 Ok(())
//             });

//         let writer = rx
//             .map_err(|()| unreachable!("rx can't fail"))
//             .fold(to_server, |to_server, msg| {
//                 to_server.send(msg)
//             })
//         .map(|_| ());

//         let _ = out_tx.clone().start_send((ClientMessage("connected".to_string()), tx.clone()));
//         reader.select(writer).map(|_| ()).map_err(|(err, _)| err)

//     });

//     core.run(socket).unwrap();
// }
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
