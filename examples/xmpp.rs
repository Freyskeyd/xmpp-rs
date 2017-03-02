
//! A simple example of hooking up stdin/stdout to a TCP stream.
//!
//! This example will connect to a server specified in the argument list and
//! then forward all data read on stdin to the server, printing out all data
//! received on stdout.
//!
//! Note that this is not currently optimized for performance, especially around
//! buffer management. Rather it's intended to show an example of working with a
//! client.

extern crate futures;
extern crate tokio_core;

use std::env;
use std::io::{self, Read, Write};
use std::net::SocketAddr;
use std::thread;

use futures::{Sink, Future, Stream};
use futures::sync::mpsc;
use tokio_core::reactor::Core;
use tokio_core::io::{Io, EasyBuf, Codec};
use tokio_core::net::TcpStream;

fn main() {
    // Parse what address we're going to connect to
    let addr = env::args().nth(1).unwrap_or("127.0.0.1:5222".to_string());
    let addr = addr.parse::<SocketAddr>().unwrap();

    // Create the event loop and initiate the connection to the remote server
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let tcp = TcpStream::connect(&addr, &handle);

    let client = tcp
        .and_then(|socket| {
            // Once the socket has been established, use the `framed` helper to
            // create a transport.
            let transport = socket.framed(Bytes);
            let h = format!("<?xml version='1.0'?>\n\
               <stream:stream xmlns:stream='http://etherx.jabber.org/streams' xmlns='jabber:client' version='1.0' to='{}'>", "localhost");
            transport.send(h.into_bytes())
        });
    // And now that we've got our client, we execute it in the event loop!
    core.run(client).unwrap();
}

/// A simple `Codec` implementation that just ships bytes around.
///
/// This type is used for "framing" a TCP stream of bytes but it's really just a
/// convenient method for us to work with streams/sinks for now. This'll just
/// take any data read and interpret it as a "frame" and conversely just shove
/// data into the output location without looking at it.
struct Bytes;

impl Codec for Bytes {
    type In = EasyBuf;
    type Out = Vec<u8>;

    fn decode(&mut self, buf: &mut EasyBuf) -> io::Result<Option<EasyBuf>> {
        if buf.len() > 0 {
            let len = buf.len();
            Ok(Some(buf.drain_to(len)))
        } else {
            Ok(None)
        }
    }

    fn encode(&mut self, data: Vec<u8>, buf: &mut Vec<u8>) -> io::Result<()> {
        buf.extend(data);
        Ok(())
    }
}
