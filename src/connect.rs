use bytes::{BytesMut, BufMut};
use futures::future;
use futures::{Future, Stream, Sink};
use native_tls::TlsConnector;
use std::net::SocketAddr;
use std::str;
use std::{io, thread};
use tokio_core::net::TcpStream;
use tokio_core::reactor::Core;
use tokio_io::codec::{Encoder, Decoder, Framed};
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_tls::TlsConnectorExt;

const START: &'static str = "<?xml version='1.0'?><stream:stream version='1.0' xmlns:stream='http://etherx.jabber.org/streams' to='example.com' xmlns='jabber:client'>";
const AUTH: &'static str = "<starttls xmlns='urn:ietf:params:xml:ns:xmpp-tls'/>";

pub fn connect_client() {
    let mut core = Core::new().unwrap();
    let addr = "127.0.0.1:5222".parse::<SocketAddr>().unwrap();

    let handle2 = core.handle();
    let handle = core.handle();
    let socket = TcpStream::connect(&addr, &core.handle())
        .and_then(|socket| {
            let transport = socket.framed(LineCodec);

            transport.send(START.to_string())
        })
        .and_then(|transport| transport.into_future().map_err(|(e, _)| e))
        .and_then(|(response, transport)| {
            transport.send(AUTH.to_string())
        })
        .and_then(|transport| transport.into_future().map_err(|(e, _)| e))
        .and_then(|(response, transport)| {
            println!("Server: {:?}", response);

            let cx = TlsConnector::builder().unwrap().build().unwrap();
            cx.connect_async(&addr.to_string(), transport.into_inner()).map_err(|e| {
                io::Error::new(io::ErrorKind::Other, e)
            })
        });

    let tls = core.run(socket).unwrap();
}


/// Our line-based codec
pub struct LineCodec;
/// Implementation of the simple line-based protocol.
///
/// Frames consist of a UTF-8 encoded string, terminated by a '\n' character.
impl Decoder for LineCodec {
    type Item = String;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<String>, io::Error> {
        // Check to see if the frame contains a new line
        // if let Some(n) = buf.as_ref().iter().position(|b| *b == b'\n') {
            // remove the serialized frame from the buffer.
        let len = buf.len();
        if len > 1 {
            let line = buf.split_to(len);

            // // Also remove the '\n'
            // buf.split_to(1);

            // Turn this data into a UTF string and return it in a Frame.
            return match str::from_utf8(&line.as_ref()) {
                Ok(s) => {
                    // println!("Receive: {}", s);
                    if s.starts_with("<?xml") {
                        let split = s.split("/><").collect::<Vec<&str>>();
                        if split.len() > 1 {
                            Ok(Some(split[0].to_string()))
                        } else {
                            Ok(None)
                        }
                    } else {
                        Ok(Some(s.to_string()))
                    }
                },
                Err(_) => Err(io::Error::new(io::ErrorKind::Other, "invalid string")),
            }
        }
        Ok(None)
        // }

        // Ok(None)
    }
}

impl Encoder for LineCodec {
    type Item = String;
    type Error = io::Error;

    fn encode(&mut self, msg: String, buf: &mut BytesMut) -> io::Result<()> {
        // Reserve enough space for the line
        // buf.reserve(msg.len() + 1);
        buf.reserve(msg.len());

        buf.extend(msg.as_bytes());
        // buf.put_u8(b'\n');

        Ok(())
    }
}

