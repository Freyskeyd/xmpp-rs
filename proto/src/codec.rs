use bytes::{BytesMut};
use std::str;
use std::{io};
use tokio_io::codec::{Encoder, Decoder};
use parser::{Parser};
use events::Event;
use events::NonStanzaEvent;
use events::StanzaEvent;
use events::IqType;

/// Our line-based codec
pub struct XMPPCodec;

impl Decoder for XMPPCodec {
    type Item = Event;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, io::Error> {
        // info!("Buffer: {:?}", str::from_utf8(buf.as_ref()));
        let (consumed, f): (usize, Option<Event>) = {
            match Parser::parse(str::from_utf8(buf.as_ref()).unwrap()) {
                Some(s) => {
                    match s {
                        Event::NonStanza(_, ref source) => (source.as_bytes().len(), Some(s.clone())),
                        Event::Stanza(_, ref source) => (source.as_bytes().len(), Some(s.clone())),
                        Event::Unknown(_, ref source) => (source.as_bytes().len(), Some(s.clone())),
                    }
                },
                None => return Ok(None)
            }
        };

        trace!("decoded string: {:?}", f);
        buf.split_to(consumed);

        Ok(f)
    }
}

impl Encoder for XMPPCodec {
    type Item = Event;
    type Error = io::Error;

    fn encode(&mut self, frame: Event, buf: &mut BytesMut) -> Result<(), Self::Error> {
        // let length = buf.len();
        trace!("will send frame: {:?}", frame);

        let f = match frame {
            Event::Unknown(event, _) => event.to_string(),
            Event::Stanza(stanza, _) => match stanza {
                StanzaEvent::Presence(event) => event.to_string(),
                StanzaEvent::IqResponse(iq_type) |
                StanzaEvent::IqRequest(iq_type) |
                StanzaEvent::Iq(iq_type) => match iq_type {
                    IqType::Bind(event) => event.to_string(),
                    IqType::Generic(event) => event.to_string()
                }
            },
            Event::NonStanza(non_stanza, _) => match non_stanza {
                NonStanzaEvent::OpenStream(event) => event.to_string(),
                NonStanzaEvent::Auth(event) => event.to_string(),
                NonStanzaEvent::ProceedTls(event) => event.to_string(),
                NonStanzaEvent::SuccessTls(event) => event.to_string(),
                NonStanzaEvent::StartTls(event) => event.to_string(),
                NonStanzaEvent::StreamFeatures(event) => event.to_string()
            }
        };

        buf.extend(f.as_bytes());
        Ok(())
    }
}
