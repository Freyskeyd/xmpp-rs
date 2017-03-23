#![deny(warnings)]
extern crate base64;
#[macro_use] extern crate futures;
extern crate tokio_core;
extern crate native_tls;
extern crate tokio_tls;
extern crate tokio_io;
extern crate bytes;


use connect::{ServerMessage};

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

mod connect;
mod codec;
mod stanza;

pub use connect::ClientMessage;
use codec::LineCodec;
use codec::ClientToServerCodec;

pub fn connect_client<F>(out_tx: mpsc::Sender<(ClientMessage, mpsc::Sender<ClientMessage>)>, f: F) 
    where F: Fn(ServerMessage) -> Option<ClientMessage> + 'static
{
    let (tx, rx) = mpsc::channel(1);
    let send_to_server = |msg| {
        match tx.clone().start_send(msg) {
            Ok(_) => println!("message sent"),
            Err(_) => println!("fail to send to sink")
        }
    };

    let domain = "xmpp-qa.iadvize.com";
    let xmpp_domain = "bot.simon.iadvize.com";
    let start = format!("<?xml version='1.0'?><stream:stream version='1.0' xmlns:stream='http://etherx.jabber.org/streams' to='{}' xmlns='jabber:client'>", xmpp_domain);
    let tls_success = format!("<stream:stream xmlns='jabber:client' xmlns:stream='http://etherx.jabber.org/streams' to='{}' version='1.0'>", xmpp_domain);


    let mut core = Core::new().unwrap();

    let stream = TcpStream::connect((domain, 5222)).unwrap();
    let socket = TokioStream::from_stream(stream, &core.handle()).unwrap();

    let transport = socket.framed(LineCodec);

    let starttls = |(response, t): (Option<String>, Framed<TokioStream, LineCodec>)| {
        println!("Response START: {:?}", response);
        t.send(stanza::non_stanza::AUTH.to_string())
            .and_then(|transport| transport.into_future().map_err(|(e, _)| e))
    };

    let negociate = |(response, transport): (Option<String>, Framed<TokioStream, LineCodec>)| {
        println!("Response STARTTLS: {:?}", response);
        let builder = TlsConnector::builder().unwrap();
        let cx = builder.build().unwrap();

        println!("connected");
        cx.connect_async(domain, transport.into_inner()).map_err(|e| {
            io::Error::new(io::ErrorKind::Other, e)
        })
    };

    let open_tls_stream = |socket: TlsStream<TokioStream>| {
        let transport = socket.framed(LineCodec);

        transport.send(tls_success.to_string())
            .and_then(|transport| transport.into_future().map_err(|(e, _)| e))
    };

    let socket = transport.send(start)
        .and_then(|transport| transport.into_future().map_err(|(e, _)| e))
        .and_then(starttls)
        .and_then(negociate)
        .and_then(open_tls_stream)
        .and_then(|(_, transport)| {
            let mut data: Vec<u8> = Vec::new();
            data.push(0);
            data.extend(b"alice@example.com");
            data.push(0);
            data.extend(b"test");

            // let plain = data.to_base64();

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
                match f(msg) {
                    Some(ret) => send_to_server(ret),
                    None => {}
                };

                Ok(())
            });

        let writer = rx
            .map_err(|()| unreachable!("rx can't fail"))
            .fold(to_server, |to_server, msg| {
                to_server.send(msg)
            })
        .map(|_| ());

        let _ = out_tx.clone().start_send((ClientMessage("connected".to_string()), tx.clone()));
        reader.select(writer).map(|_| ()).map_err(|(err, _)| err)

    });

    core.run(socket).unwrap();
}
