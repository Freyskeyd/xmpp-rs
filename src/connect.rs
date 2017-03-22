use bytes::{BytesMut};
use std::str;
use std::{io};
use tokio_io::codec::{Encoder, Decoder};
use std::marker::PhantomData;

pub const AUTH: &'static str = "<starttls xmlns='urn:ietf:params:xml:ns:xmpp-tls'/>";
pub const PLAIN: &'static str = "<auth xmlns='urn:ietf:params:xml:ns:xmpp-sasl' mechanism='PLAIN'>";

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

