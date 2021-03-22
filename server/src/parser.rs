use bytes::BufMut;
use circular::Buffer;
use log::trace;
use std::{
    borrow::Cow,
    io::{self, Write},
};
use tokio_util::codec::Decoder;
use tokio_util::codec::Encoder;
use uuid::Uuid;
use xml::{attribute::OwnedAttribute, name::OwnedName, namespace::Namespace, reader::ErrorKind as XmlErrorKind};
use xml::{common::Position, reader::XmlEvent, EventReader, ParserConfig};
use xmpp_proto::NonStanza;
use xmpp_proto::{ns, FromXmlElement, GenericIq, OpenStreamBuilder, ProceedTls};
use xmpp_proto::{Packet, StartTls};
use xmpp_xml::Element;

/// XmppCodec deals with incoming bytes. You can feed the parser with bytes and try to detect new
/// event.
pub struct XmppCodec {
    sink: PacketSink,
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
        trace!("");
        trace!("==================================================");
        let _ = self.sink.parser.source_mut().write(&buf[..]);

        trace!("Buffer contains: {}", String::from_utf8_lossy(self.sink.parser.source().data()));
        trace!("");
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

struct PacketSink {
    pub parser: EventReader<Buffer>,
}

impl PacketSink {
    fn reset(&mut self, saved_buffer: &[u8]) {
        let mut cfg = ParserConfig::new().whitespace_to_characters(true);
        cfg.ignore_end_of_stream = true;
        self.parser = {
            let mut source = Buffer::with_capacity(4096);
            // println!("BEFORE RESET {:?}", String::from_utf8_lossy(self.parser.source().data()));
            let _ = source.write_all(saved_buffer);
            // source.clone_from(saved_buffer);
            let parser = EventReader::new_with_config(source, cfg);

            parser
        };
        println!("PARSER RESETED");

        println!("     -> parse pos {:?}", self.parser.position());
        println!("     -> available: {:?}", self.parser.source().available_data());
    }

    fn parse_start_element(&mut self, name: OwnedName, namespace: Namespace, attributes: Vec<OwnedAttribute>) -> Option<Packet> {
        if name.local_name == "auth" && name.namespace_ref() == Some(ns::SASL) {
            if let Ok(e) = Element::from_start_element(name, attributes, namespace, None, &mut self.parser) {
                let e = ProceedTls::from_element(e).unwrap();
                // TODO VALIDATE AUTH
                return Some(Packet::NonStanza(NonStanza::ProceedTls(e)));
            }
        } else if name.local_name == "starttls" && name.namespace_ref() == Some(ns::TLS) {
            if Element::from_start_element(name, attributes, namespace, None, &mut self.parser).is_ok() {
                let e = StartTls {};
                return Some(Packet::NonStanza(NonStanza::StartTls(e)));
            }
        } else if name.local_name == "iq" {
            if let Ok(e) = Element::from_start_element(name, attributes, namespace, None, &mut self.parser) {
                let iq = GenericIq::from_element(e).unwrap();
                println!("DETECTE IQ {:?}", iq);
                return None;
                // return Some(Packet::Stanza(Stanza::IQ(iq)));
            }
        }
        None
    }

    pub fn next_packet(&mut self) -> Option<Packet> {
        // Using loop for now but can be removed soon I think
        loop {
            trace!("next_event");
            trace!("     -> parse pos {:?}", self.parser.position());
            trace!("     -> available: {:?}", self.parser.source().available_data());

            trace!("Buffer contains: {}", String::from_utf8_lossy(self.parser.source().data()));
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
                        trace!("     -> Start Document");
                        continue;
                    }
                    // Dealing with the openning stream processus
                    // This kind of XML isn't close until the end of the stream, we can't use
                    // default behaviour for this.
                    XmlEvent::StartElement { ref name, namespace, attributes } if name.local_name == "stream" && name.namespace_ref() == Some(ns::STREAM) => {
                        trace!("     -> Start stream:stream");

                        let (to, lang, version) = attributes.iter().fold((String::from(""), String::from("en"), String::from("0.0")), |(to, lang, version), attribute| {
                            match attribute.name.local_name.as_ref() {
                                "to" if attribute.name.namespace.is_none() => (attribute.value.to_string(), lang, version),
                                "lang" if attribute.name.namespace == Some(ns::XML_URI.to_string()) => (to, attribute.value.to_string(), version),
                                "version" if attribute.name.namespace.is_none() => (to, lang, attribute.value.to_string()),
                                _ => (to, lang, version),
                            }
                        });
                        let e = OpenStreamBuilder::default()
                            .id(Uuid::new_v4())
                            .to(to)
                            .lang(lang)
                            .version(version)
                            .xmlns(namespace.get("").unwrap_or(ns::CLIENT))
                            .build()
                            .unwrap();

                        return Some(Packet::NonStanza(NonStanza::OpenStream(e)));
                    }

                    XmlEvent::StartElement { name, namespace, attributes } => {
                        if let Some(e) = self.parse_start_element(name, namespace, attributes) {
                            return Some(e);
                        }
                    }

                    e => {
                        trace!("----------> Hit something");
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