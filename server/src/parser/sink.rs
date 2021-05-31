use circular::Buffer;
use log::{error, trace};
use std::{borrow::Cow, io::Write};
use xml::{attribute::OwnedAttribute, name::OwnedName, namespace::Namespace, reader::ErrorKind as XmlErrorKind};
use xml::{reader::XmlEvent, EventReader, ParserConfig};
use xmpp_proto::{ns, CloseStream, Packet, PacketParsingError};

pub struct PacketSink {
    pub parser: EventReader<Buffer>,
}

impl PacketSink {
    fn reset(&mut self, saved_buffer: &[u8]) {
        trace!("RESETTING stream");
        let mut cfg = ParserConfig::new().whitespace_to_characters(true);
        cfg.ignore_end_of_stream = true;
        self.parser = {
            let mut source = Buffer::with_capacity(4096);
            let _ = source.write_all(saved_buffer);
            EventReader::new_with_config(source, cfg)
        };
    }

    fn parse_start_element(&mut self, name: OwnedName, namespace: Namespace, attributes: Vec<OwnedAttribute>) -> Result<Packet, PacketParsingError> {
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
                        if let Ok(e) = self.parse_start_element(name, namespace, attributes) {
                            return Some(e);
                        }
                    }

                    XmlEvent::EndElement { name } => {
                        if name == OwnedName::qualified("stream", ns::STREAM, Some("stream")) {
                            return Some(CloseStream {}.into());
                        } else {
                            break;
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
                    error!("Error {:?}", e);
                    break;
                }
            }
        }

        None
    }
}
