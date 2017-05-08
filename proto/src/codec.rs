use bytes::{BytesMut};
use std::str;
use std::{io};
use tokio_io::codec::{Encoder, Decoder};
// use parser::{Parser};
// use events;
use events::Event;
use events::Event::*;
use events::NonStanzaEvent::*;
use events::StanzaEvent::*;
use events::IqEvent::*;
// use config::XMPPConfig;
// use events::EventTrait;
// use elementtree::Element;
// use xml::reader::{EventReader,XmlEvent};
// use xml::reader::EventReader;
// use xml::common::Position;
// use ns;
// use std::io::{Read};
// use tokio_core::net::TcpStream;
use parser::XmppParser;
/// Our line-based codec
pub struct XMPPCodec {
    pub parser: XmppParser
}

impl XMPPCodec {
    pub fn new() -> XMPPCodec {
        XMPPCodec {
            parser: XmppParser::new()
        }
    }
}

impl Decoder for XMPPCodec {
    type Item = Event;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, io::Error> {

        trace!("");
        trace!("==================================================");
        self.parser.feed(&buf[..]);

        trace!("Buffer contains: {}", str::from_utf8(self.parser.source().data()).unwrap());
        trace!("");
        let event = match self.parser.next_event() {
            Some(e) => {
                trace!("Decode: event: {:?}", e);
                Some(e)
            },
            _ => None
        };
        let l = buf.len();
        buf.split_to(l);
        Ok(event)
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
