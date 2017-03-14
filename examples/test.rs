
extern crate futures;
extern crate tokio_core;
extern crate xmpp;

use std::thread;

use std::net::SocketAddr;
use tokio_core::io::Io;
use tokio_core::reactor::Core;
use tokio_core::net::TcpStream;
use futures::{Stream, Sink, Future};
use futures::sync::mpsc;
use futures::sync::mpsc::Sender;
use xmpp::{Handshake, HandshakeCodec, ClientMessage, ServerMessage,
                        ClientToServerCodec};

fn main() {
    run_client(rx);
}

fn run_client(rx) {
    let addr = "127.0.0.1:5222".parse::<SocketAddr>().unwrap();

    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let tcp = TcpStream::connect(&addr, &handle);

    let handshake = tcp.and_then(|stream| {
        let handshake_io = stream.framed(HandshakeCodec::new());

        let start = "<?xml version='1.0'?><stream:stream version='1.0' xmlns:stream='http://etherx.jabber.org/streams' to='example.com' xmlns='jabber:client'>";
        handshake_io
            .send(start.to_string())
            .map(|handshake_io| handshake_io.into_inner())
    });

    let client = handshake.and_then(|socket| {
        let (to_server, from_server) = socket.framed(ClientToServerCodec::new()).split();

        let reader = from_server
            .for_each(move |msg| {
                if msg.len() > 1 {
                    println!("{}", msg);
                }
                Ok(())
            });

        let writer = rx
            .map_err(|()| unreachable!("rx can't fail"))
            .fold(to_server, |to_server, msg| {
                to_server.send(msg)
            })
            .map(|_| ());

        reader.select(writer).map(|_| ()).map_err(|(err, _)| err)
    });

    core.run(client).unwrap();
}


// Handshake message sent from a client to a server when it first connects, identifying the
// username of the client.
#[derive(Debug, Clone)]
pub struct Handshake {
    pub name: String,
}

impl Handshake {
    pub fn new<S: Into<String>>(name: S) -> Handshake {
        Handshake { name: name.into() }
    }
}

pub type HandshakeCodec = codec::LengthPrefixedJson;

#[derive(Debug, Clone)]
pub struct ClientMessage(pub String);

impl ClientMessage {
    pub fn new<S: Into<String>>(message: S) -> ClientMessage {
        ClientMessage(message.into())
    }
}

// Enumerate possible messages the server can send to clients.
#[derive(Debug, Clone)]
pub enum ServerMessage {
    // A message from a client (first String) containing arbitrary content (second String).
    Message(String, String),

    // Notification of a new user connection. The associated String is the name that user provided
    // in their Handshake.
    UserConnected(String),
    //
    // Notification of user disconnection. The associated String is the name that user provided
    // in their Handshake.
    UserDisconnected(String),
}

pub type ServerToClientCodec = codec::LengthPrefixedJson;
pub type ClientToServerCodec = codec::LengthPrefixedJson;
