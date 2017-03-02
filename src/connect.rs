use bytes::{BytesMut};
use std::str;
use std::{io};
use tokio_io::codec::{Encoder, Decoder};
use std::marker::PhantomData;

pub const AUTH: &'static str = "<starttls xmlns='urn:ietf:params:xml:ns:xmpp-tls'/>";
pub const PLAIN: &'static str = "<auth xmlns='urn:ietf:params:xml:ns:xmpp-sasl' mechanism='PLAIN'>";

/// Our line-based codec
pub struct LineCodec;
/// Implementation of the simple line-based protocol.
///
/// Frames consist of a UTF-8 encoded string, terminated by a '\n' character.
impl Decoder for LineCodec {
    type Item = String;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<String>, io::Error> {
        let len = buf.len();
        if len > 1 {
            println!("IN: {:?}", str::from_utf8(buf.as_ref()));
            let line = buf.split_to(len);

            return match str::from_utf8(&line.as_ref()) {
                Ok(s) => {
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
    }
}

impl Encoder for LineCodec {
    type Item = String;
    type Error = io::Error;

    fn encode(&mut self, msg: String, buf: &mut BytesMut) -> io::Result<()> {
        buf.reserve(msg.len());

        buf.extend(msg.as_bytes());

        Ok(())
    }
}


#[derive(Debug, Clone)]
pub struct Handshake {
    pub name: String,
}

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

impl Decoder for LengthPrefixedJson
{
    type Item = ServerMessage;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, io::Error> {
        let len = buf.len();
        if len < 1 {
            return Ok(None);
        }

        let buf = buf.split_to(len);
        let s = str::from_utf8(buf.as_ref()).unwrap();

        println!("IN: {:?}", s);
        Ok(Some(ServerMessage(s.to_string())))
    }
}
impl Encoder for LengthPrefixedJson {
    type Item = ClientMessage;
    type Error = io::Error;


    fn encode(&mut self, msg: ClientMessage, buf: &mut BytesMut) -> io::Result<()> {
        println!("OUT: {:?}", msg.0);
        buf.extend(msg.0.as_bytes());

        Ok(())
    }
}

// pub type ServerToClientCodec = LengthPrefixedJson;
pub type ClientToServerCodec = LengthPrefixedJson;
