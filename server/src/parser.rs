use bytes::BufMut;
use circular::Buffer;
use log::trace;
use std::{
    borrow::Cow,
    io::{self, Write},
};
use tokio_util::codec::Decoder;
use tokio_util::codec::Encoder;
use xml::{attribute::OwnedAttribute, name::OwnedName, namespace::Namespace, reader::ErrorKind as XmlErrorKind};
use xml::{reader::XmlEvent, EventReader, ParserConfig};
use xmpp_proto::Packet;

/// XmppCodec deals with incoming bytes. You can feed the parser with bytes and try to detect new
/// event.
pub struct XmppCodec {
    pub sink: PacketSink,
}

impl XmppCodec {
    pub fn new() -> Self {
        Self { sink: Self::new_sink() }
    }

    fn new_sink() -> PacketSink {
        let mut cfg = ParserConfig::new().whitespace_to_characters(true);
        cfg.ignore_end_of_stream = true;
        PacketSink {
            parser: cfg.create_reader(Buffer::with_capacity(4096)),
        }
    }
}

impl Decoder for XmppCodec {
    type Item = Packet;

    type Error = io::Error;

    fn decode(&mut self, buf: &mut bytes::BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let _ = self.sink.parser.source_mut().write(&buf[..]);
        if !self.sink.parser.source().data().is_empty() {
            trace!("Buffer contains: {}", String::from_utf8_lossy(self.sink.parser.source().data()));
        }
        let event = match self.sink.next_packet() {
            Some(e) => {
                trace!("Decoded Packet: {:?}", e);
                Some(e)
            }
            _ => None,
        };
        let l = buf.len();
        let _ = buf.split_to(l);
        Ok(event)
    }
}

impl Encoder<Packet> for XmppCodec {
    type Error = io::Error;

    fn encode(&mut self, item: Packet, dst: &mut bytes::BytesMut) -> Result<(), Self::Error> {
        let _ = item.write_to_stream(dst.writer());

        Ok(())
    }
}

pub struct PacketSink {
    pub parser: EventReader<Buffer>,
}

impl PacketSink {
    fn reset(&mut self, saved_buffer: &[u8]) {
        let mut cfg = ParserConfig::new().whitespace_to_characters(true);
        cfg.ignore_end_of_stream = true;
        self.parser = {
            let mut source = Buffer::with_capacity(4096);
            let _ = source.write_all(saved_buffer);
            EventReader::new_with_config(source, cfg)
        };
    }

    fn parse_start_element(&mut self, name: OwnedName, namespace: Namespace, attributes: Vec<OwnedAttribute>) -> Option<Packet> {
        Packet::parse(&mut self.parser, name, namespace, attributes)
    }

    pub fn next_packet(&mut self) -> Option<Packet> {
        // Using loop for now but can be removed soon I think
        loop {
            let saved_buffer = self.parser.source().data().to_vec();
            // Stop loop if buffer is empty
            if self.parser.source().available_data() == 0 {
                return None;
            }

            // Reopen the parser to check new bytes
            // self.parser.reopen_parser();
            match self.parser.next() {
                Ok(xml_event) => match xml_event {
                    XmlEvent::StartDocument { .. } => {
                        continue;
                    }
                    XmlEvent::StartElement { name, namespace, attributes } => {
                        if let Some(e) = self.parse_start_element(name, namespace, attributes) {
                            return Some(e);
                        }
                    }

                    e => {
                        trace!("{:?}", e);
                        continue;
                    }
                },
                // --> Server return <?xml version=\'1.0\'?> but it fail our parser
                // Err(ref e) if e.kind().eq(&XmlErrorKind::Syntax(Cow::from("Unexpected token: <?version"))) => continue,
                // Err(ref e) if e.kind().eq(&XmlErrorKind::Syntax(Cow::from("Unexpected token: <?version="))) => continue,
                // Err(ref e) if e.kind().eq(&XmlErrorKind::Syntax(Cow::from("Unexpected token: <?version\'"))) => continue,
                // Err(ref e) if e.kind().eq(&XmlErrorKind::Syntax(Cow::from("Unexpected token: <?version1.0\'"))) => continue,
                Err(ref e) if e.kind().eq(&XmlErrorKind::Syntax(Cow::from("Invalid processing instruction: <?xml"))) => {
                    self.reset(&saved_buffer);
                    continue;
                }
                // Err(ref e) if e.kind().eq(&XmlErrorKind::Syntax(Cow::from("Unexpected end of stream: still inside the root element"))) => break,
                Err(e) => {
                    trace!("Error {:?}", e);
                    break;
                }
            }
        }

        None
    }
}
