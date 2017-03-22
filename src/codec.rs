use bytes::{BytesMut};
use std::str;
use std::{io};
use tokio_io::codec::{Encoder, Decoder};
use ::connect::LengthPrefixedJson;
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

pub type ClientToServerCodec = LengthPrefixedJson;
// pub type ServerToClientCodec = LengthPrefixedJson;
