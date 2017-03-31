use bytes::{BytesMut};
use std::str;
use std::{io};
use tokio_io::codec::{Encoder, Decoder};
use parser::{MessageType, NonStanzaType, StanzaType};

/// Our line-based codec
pub struct XMPPCodec;

type Frame = String;

impl Decoder for XMPPCodec {
    type Item = Frame;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Frame>, io::Error> {
        trace!("Buffer: {:?}", str::from_utf8(buf.as_ref()));
        let (consumed, f): (usize, Option<String>) = {
            let mut stanzas = MessageType::parse(str::from_utf8(buf.as_ref()).unwrap());
            if stanzas.len() > 0 {
                match stanzas[0] {
                    MessageType::Unknown(ref mut raw) => {
                        (raw.inner.as_bytes().len(), Some(String::new()))
                    },
                    MessageType::NonStanza(NonStanzaType::ProceedTls(ref mut raw)) |
                        MessageType::NonStanza(NonStanzaType::SuccessTls(ref mut raw)) |
                        MessageType::NonStanza(NonStanzaType::StreamOpen(ref mut raw)) |
                        MessageType::NonStanza(NonStanzaType::StreamFeatures(ref mut raw)) |
                        MessageType::Stanza(StanzaType::Iq(ref mut raw)) => (raw.inner.as_bytes().len(), Some(raw.inner.to_string())),
                    _ => return Ok(None)
                }
            } else {
                return Ok(None)
            }
        };

        trace!("decoded string: {:?}", f);
        buf.split_to(consumed);

        Ok(f)
    }
}

impl Encoder for XMPPCodec {
    type Item = Frame;
    type Error = io::Error;

    fn encode(&mut self, frame: Frame, buf: &mut BytesMut) -> Result<(), Self::Error> {
        // let length = buf.len();
        trace!("will send frame: {:?}", frame);

        buf.extend(frame.as_bytes());
        Ok(())
            // loop {
            //     return Ok(())
            // let gen_res = match &frame {
            //   &Frame::ProtocolHeader => {
            //     gen_protocol_header((buf, 0)).map(|tup| tup.1)
            //   },
            //   &Frame::Heartbeat(_) => {
            //     gen_heartbeat_frame((buf, 0)).map(|tup| tup.1)
            //   },
            //   &Frame::Method(channel, ref method) => {
            //     gen_method_frame((buf, 0), channel, method).map(|tup| tup.1)
            //   },
            //   &Frame::Header(channel_id, class_id, ref header) => {
            //     gen_content_header_frame((buf, 0), channel_id, class_id, header.body_size).map(|tup| tup.1)
            //   },
            //   &Frame::Body(channel_id, ref data) => {
            //     gen_content_body_frame((buf, 0), channel_id, data).map(|tup| tup.1)
            //   }
            // };

            // match gen_res {
            //   Ok(sz) => {
            //     buf.truncate(sz);
            //     trace!("serialized frame: {} bytes", sz);
            //     return Ok(());
            //   },
            //   Err(e) => {
            //     error!("error generating frame: {:?}", e);
            //     match e {
            //       GenError::BufferTooSmall(sz) => {
            //         buf.extend(repeat(0).take(sz - length));
            //         //return Err(Error::new(ErrorKind::InvalidData, "send buffer too small"));
            //       },
            //       GenError::InvalidOffset | GenError::CustomError(_) | GenError::NotYetImplemented => {
            //         return Err(Error::new(ErrorKind::InvalidData, "could not generate"));
            //       }
            //     }
            //   }
            // }
            // }
    }
}
