use bytes::{BytesMut};
use std::str;
use std::{io};
use tokio_io::codec::{Encoder, Decoder};
use parser::{Parser};
use events::Event;
use events::Event::*;
use events::NonStanzaEvent::*;
use events::StanzaEvent::*;
use events::IqType::*;

/// Our line-based codec
pub struct XMPPCodec;

impl Decoder for XMPPCodec {
    type Item = Event;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, io::Error> {

        trace!("Buffer contain: {:?}", str::from_utf8(buf.as_ref()));
        let (consumed, f): (usize, Option<Event>) = {
            match Parser::parse(str::from_utf8(buf.as_ref()).unwrap()) {
                Some(s) => {
                    match s {
                        NonStanza(_, ref source) => (source.as_bytes().len(), Some(s.clone())),
                        Stanza(_, ref source) => (source.as_bytes().len(), Some(s.clone())),
                        Unknown(_, ref source) => (source.as_bytes().len(), Some(s.clone())),
                    }
                },
                None => return Ok(None)
            }
        };

        buf.split_to(consumed);

        Ok(f)
    }
}

impl Encoder for XMPPCodec {
    type Item = Event;
    type Error = io::Error;

    fn encode(&mut self, frame: Event, buf: &mut BytesMut) -> Result<(), Self::Error> {
        trace!("will send frame: {:?}", frame);

        let f = match frame {
            Unknown(event, _) => event.to_string(),
            Stanza(stanza, _) => match stanza {
                PresenceEvent(event) => event.to_string(),
                MessageEvent(event) => event.to_string(),
                IqResponseEvent(iq_type) |
                IqRequestEvent(iq_type) |
                IqEvent(iq_type) => match iq_type {
                    PingIq(event) => event.to_string(),
                    BindIq(event) => event.to_string(),
                    GenericIq(event) => event.to_string()
                }
            },
            NonStanza(non_stanza, _) => match non_stanza {
                OpenStreamEvent(event) => event.to_string(),
                AuthEvent(event) => event.to_string(),
                ProceedTlsEvent(event) => event.to_string(),
                SuccessTlsEvent(event) => event.to_string(),
                StartTlsEvent(event) => event.to_string(),
                StreamFeaturesEvent(event) => event.to_string()
            }
        };

        buf.extend(f.as_bytes());
        Ok(())
    }
}
