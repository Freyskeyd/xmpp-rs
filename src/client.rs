use std::io;
use std::net::ToSocketAddrs;

use futures::Future;
use native_tls::TlsConnector;
use tokio_core::net::TcpStream;
use tokio_core::reactor::Core;
use tokio_tls::TlsConnectorExt;


use std::str::FromStr;
use std::fmt::Debug;
use tokio_core::io::{Codec, EasyBuf};
use std::str;
use std::io::{Read, Write};
use std::net::SocketAddr;
use std::marker::PhantomData;
use tokio_core::io::Io;
use futures::{Stream, Sink};
use futures::sync::mpsc;
use tokio_io::AsyncWrite;
use tokio_io::AsyncRead;
use tokio_tls::TlsStream;

// pub fn run_client<F>(stream: TlsStream<>, rx: mpsc::Receiver<ClientMessage>, tx: mpsc::Sender<ClientMessage>, f: F) 
//     where 
//     //S: AsyncRead + AsyncWrite,
//     F: Fn(ServerMessage) -> Option<ClientMessage> + 'static
// {
//     let send_to_server = |msg| {
//         match tx.clone().start_send(msg) {
//             Ok(_) => println!("message sent"),
//             Err(_) => println!("fail to send to sink")
//         }
//     };
//     let (to_server, from_server) = stream.framed(ClientToServerCodec::new()).split();

//     let reader = from_server
//         .for_each(move |msg| {
//             if msg.0.len() > 1 {
//                 if msg.0.starts_with("<stream:features") {

//                     let message = String::from("<starttls xmlns='urn:ietf:params:xml:ns:xmpp-tls'/>");

//                     send_to_server(ClientMessage(message));
//                 }
//             }

//             match f(msg) {
//                 Some(ret) => send_to_server(ret),
//                 None => {}
//             };

//             Ok(())
//         });

//     let writer = rx
//         .map_err(|()| unreachable!("rx can't fail"))
//         .fold(to_server, |to_server, msg| {
//             to_server.send(msg)
//         })
//     .map(|_| ());

//     reader.select(writer).map(|_| ()).map_err(|(err, _)| err)
// }

pub fn run_client<F>(rx: mpsc::Receiver<ClientMessage>, tx: mpsc::Sender<ClientMessage>, f: F) 
    where F: Fn(ServerMessage) -> Option<ClientMessage> + 'static
{
}
    // let addr = "127.0.0.1:5222".parse::<SocketAddr>().unwrap();

    // let mut core = Core::new().unwrap();
    // let handle = core.handle();
    // let tcp = TcpStream::connect(&addr, &handle);

    // let handshake = tcp.and_then(|stream| {
    //     let handshake_io = stream.framed(HandshakeCodec::new());

    //     let start = "<?xml version='1.0'?><stream:stream version='1.0' xmlns:stream='http://etherx.jabber.org/streams' to='example.com' xmlns='jabber:client'>";
    //     handshake_io
    //         .send(ClientMessage(start.to_string()))
    //         .map(|handshake_io| handshake_io.into_inner())
    // });

  

    // let client = handshake.and_then(|socket| {
    //     let (to_server, from_server) = socket.framed(ClientToServerCodec::new()).split();

    //     let reader = from_server
    //         .for_each(move |msg| {
    //             if msg.0.len() > 1 {
    //                 if msg.0.starts_with("<stream:features") {

    //                 let message = String::from("<starttls xmlns='urn:ietf:params:xml:ns:xmpp-tls'/>");

    //                 send_to_server(ClientMessage(message));
    //                 }
    //             }

    //             match f(msg) {
    //                 Some(ret) => send_to_server(ret),
    //                 None => {}
    //             };

    //             Ok(())
    //         });

    //     let writer = rx
    //         .map_err(|()| unreachable!("rx can't fail"))
    //         .fold(to_server, |to_server, msg| {
    //             to_server.send(msg)
    //         })
    //         .map(|_| ());

    //     reader.select(writer).map(|_| ()).map_err(|(err, _)| err)
    // });

    // core.run(client).unwrap();
// }

use std::fmt;
#[derive(Debug, Clone)]
pub struct Handshake {
    pub name: String,
}

impl Handshake {
    pub fn new<S: Into<String>>(name: S) -> Handshake {
        Handshake { name: name.into() }
    }
}

pub type HandshakeCodec = LengthPrefixedJson;

#[derive(Debug, Clone)]
pub struct ClientMessage(pub String);

#[derive(Debug)]
pub struct ServerMessage(pub String);


pub struct LengthPrefixedJson{
    _in: PhantomData<ServerMessage>,
    _out: PhantomData<ClientMessage>,
}

impl LengthPrefixedJson {
    pub fn new() -> LengthPrefixedJson{
        LengthPrefixedJson {
            _in: PhantomData,
            _out: PhantomData,
        }
    }
}


impl Codec for LengthPrefixedJson
{
    type In = ServerMessage;
    type Out = ClientMessage;

    fn decode(&mut self, buf: &mut EasyBuf) -> io::Result<Option<Self::In>> {
        let len = buf.len();
        if len < 1 {
            return Ok(None);
        }

        let buf = buf.drain_to(len);
        let s = str::from_utf8(buf.as_slice()).unwrap();

        println!("IN: {:?}", s);
        Ok(Some(ServerMessage(s.to_string())))
    }

    fn encode(&mut self, msg: ClientMessage, buf: &mut Vec<u8>) -> io::Result<()> {
        println!("OUT: {:?}", msg);
        let _ = buf.write(msg.0.to_string().as_bytes());

        Ok(())
    }
}


// // Enumerate possible messages the server can send to clients.
// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub enum ServerMessage {
//     // A message from a client (first String) containing arbitrary content (second String).
//     Message(String, String),

//     // Notification of a new user connection. The associated String is the name that user provided
//     // in their Handshake.
//     UserConnected(String),
//     //
//     // Notification of user disconnection. The associated String is the name that user provided
//     // in their Handshake.
//     UserDisconnected(String),
// }

pub type ServerToClientCodec = LengthPrefixedJson;
pub type ClientToServerCodec = LengthPrefixedJson;
