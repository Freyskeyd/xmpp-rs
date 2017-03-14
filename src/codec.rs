use tokio_core::io::{Codec, EasyBuf};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use std::str;
use std::io::Write;
use std::io;
use std::marker::PhantomData;
use std::mem;

pub struct LengthPrefixedJson
{
    _in: PhantomData<String>,
    _out: PhantomData<String>,
}

impl LengthPrefixedJson
{
    pub fn new() -> LengthPrefixedJson {
        LengthPrefixedJson {
            _in: PhantomData,
            _out: PhantomData,
        }
    }
}

// `LengthPrefixedJson` is a codec for sending and receiving serde_json serializable types. The
// over the wire format is a Big Endian u16 indicating the number of bytes in the JSON payload
// (not including the 2 u16 bytes themselves) followed by the JSON payload.
impl Codec for LengthPrefixedJson
{
    type In = String;
    type Out = String;

    fn decode(&mut self, buf: &mut EasyBuf) -> io::Result<Option<Self::In>> {
        // Make sure we have at least the 2 u16 bytes we need.
        // let msg_size = match buf.as_ref().read_u16::<BigEndian>() {
        //     Ok(msg_size) => msg_size,
        //     Err(_) => return Ok(None),
        // };
        // let hdr_size = mem::size_of_val(&msg_size);
        // let msg_size = msg_size as usize + hdr_size;

        // Make sure our buffer has all the bytes indicated by msg_size.
        let len = buf.len();
        if len < 1 {
            return Ok(None);
        }

        // Drain off the entire message.
        let buf = buf.drain_to(len);
        // Decode!
        let s = str::from_utf8(buf.as_slice()).unwrap();
        Ok(Some(s.to_string()))
    }

    fn encode(&mut self, msg: String, buf: &mut Vec<u8>) -> io::Result<()> {
        // Encode directly into `buf`.
        buf.write(msg.as_bytes());

        Ok(())
    }
}
