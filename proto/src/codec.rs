use bytes::{BytesMut};
use std::str;
use std::{io};
use tokio_io::codec::{Encoder, Decoder};
use parser::{Parser};
use events::Event;
use events::Event::*;
use events::NonStanzaEvent::*;
use events::StanzaEvent::*;
use events::IqEvent::*;

/// Our line-based codec
pub struct XMPPCodec;

impl Decoder for XMPPCodec {
    type Item = Event;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, io::Error> {

        let (consumed, f): (usize, Option<Event>) = {
            match Parser::parse(str::from_utf8(buf.as_ref()).unwrap()) {
                Some((s, source)) => {
                    (source.as_bytes().len(), Some(s.clone()))
                    // match s {
                    //     NonStanza(_, _) |
                    //     Stanza(_, _) |
                    //     Unknown(_, _) => (source.as_bytes().len(), Some(s.clone())),
                    // }
                },
                None => return Ok(None)
            }
        };

        buf.split_to(consumed);

        trace!("extract from buffer: {:?}", f);
        trace!("remained in buffer: {}", str::from_utf8(buf.as_ref()).unwrap());
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
            Stanza(stanza, _) => match *stanza {
                PresenceEvent(event) => event.to_string(),
                MessageEvent(event) => event.to_string(),
                IqResponseEvent(boxed_iq) |
                IqRequestEvent(boxed_iq) => match *boxed_iq {
                    PingEvent(event) => event.to_string(),
                    BindEvent(event) => event.to_string(),
                    GenericEvent(event) => event.to_string()
                },
                IqEvent(iq_event) => match *iq_event {
                    PingEvent(event) => event.to_string(),
                    BindEvent(event) => event.to_string(),
                    GenericEvent(event) => event.to_string()
                }
            },
            NonStanza(non_stanza, _) => match *non_stanza {
                CloseStreamEvent => String::from("</stream:stream>"),
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
