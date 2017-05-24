use std::io::Write;
use events::*;
use ns;
use std::str;
use std::borrow::Cow;
use circular::Buffer;
use config::XMPPConfig;
use xml::reader::{EventReader, ParserConfig, XmlEvent};
use xml::attribute::OwnedAttribute;
use xml::namespace::Namespace;
use xml::name::OwnedName;
use xml::common::Position;
// use std::io::Read;
use xml::reader::ErrorKind as XmlErrorKind;
use xmpp_xml::Element;

pub struct XmppParser {
    pub parser: EventReader<Buffer>,
}

impl Default for XmppParser {
    fn default() -> Self {
        Self::new()
    }
}

impl XmppParser {
    pub fn source(&self) -> &Buffer {
        self.parser.source()
    }
    pub fn new() -> XmppParser {
        let mut cfg = ParserConfig::new().whitespace_to_characters(true);
        cfg.ignore_end_of_stream = true;
        XmppParser { parser: cfg.create_reader(Buffer::with_capacity(4096)) }
    }

    fn parse_start_element(&mut self, name: OwnedName, namespace: Namespace, attributes: Vec<OwnedAttribute>) -> Option<Event> {
        if name.local_name == "features" && name.namespace_ref() == Some(ns::STREAM) {
            trace!("     -> Start stream:features");
            if let Ok(e) = Element::from_start_element(name, attributes, namespace, None, &mut self.parser) {
                if let Ok(e) = StreamFeatures::from_element(e) {
                    return Some(e.to_event());
                }
            }
        } else if name.local_name == "proceed" && name.namespace_ref() == Some(ns::TLS) {
            if Element::from_start_element(name, attributes, namespace, None, &mut self.parser).is_ok() {
                let e = ProceedTls::new(&XMPPConfig::new());
                return Some(e.to_event());
            }
        } else if name.local_name == "success" && name.namespace_ref() == Some(ns::SASL) {
            if Element::from_start_element(name, attributes, namespace, None, &mut self.parser).is_ok() {
                let e = SuccessTls::new(&XMPPConfig::new());

                return Some(e.to_event());
            }
        } else if name.local_name == "iq" {
            if let Ok(e) = Element::from_start_element(name, attributes, namespace, None, &mut self.parser) {
                if let Ok(e) = GenericIq::from_element(e) {
                    return Some(e.to_event());
                }
            }
        } else if name.local_name == "message" {
            if let Ok(e) = Element::from_start_element(name, attributes, namespace, None, &mut self.parser) {
                if let Ok(e) = GenericMessage::from_element(e) {
                    return Some(e.to_event());
                }
            }
        }
        None
    }

    pub fn next_event(&mut self) -> Option<Event> {
        // Using loop for now but can be removed soon I think
        loop {
            trace!("next_event");
            trace!("     -> parse pos {:?}", self.parser.position());
            trace!("     -> available: {:?}",
                   self.parser.source().available_data());


            // Stop loop if buffer is empty
            if self.parser.source().available_data() == 0 {
                return None;
            }

            // Reopen the parser to check new bytes
            // self.parser.reopen_parser();
            match self.parser.next() {
                Ok(xml_event) => {
                    match xml_event {
                        XmlEvent::StartDocument { .. } => {
                            trace!("     -> Start Document");
                            continue;
                        }
                        XmlEvent::StartElement { ref name, .. } if name.local_name == "stream" && name.namespace_ref() == Some(ns::STREAM) => {
                            trace!("     -> Start stream:stream");
                            let e = OpenStream::new(&XMPPConfig::new());

                            return Some(e.to_event());
                        }
                        XmlEvent::StartElement {
                            name,
                            namespace,
                            attributes,
                        } => {
                            if let Some(e) = self.parse_start_element(name, namespace, attributes) {
                                return Some(e);
                            }
                        }

                        e => {
                            trace!("----------> Hit something");
                            trace!("{:?}", e);
                            continue;
                        }
                    }
                }
                // --> Server return <?xml version=\'1.0\'?> but it fail our parser
                Err(ref e) if e.kind()
                                  .eq(&XmlErrorKind::Syntax(Cow::from("Unexpected token: <?version"))) => continue,
                Err(ref e) if e.kind()
                                  .eq(&XmlErrorKind::Syntax(Cow::from("Unexpected token: <?version="))) => continue,
                Err(ref e) if e.kind()
                                  .eq(&XmlErrorKind::Syntax(Cow::from("Unexpected token: <?version\'"))) => continue,
                Err(ref e) if e.kind()
                                  .eq(&XmlErrorKind::Syntax(Cow::from("Unexpected token: <?version1.0\'"))) => continue,
                Err(ref e) if e.kind()
                                  .eq(&XmlErrorKind::Syntax(Cow::from("Invalid processing instruction: <?xml"))) => continue,
                Err(e) => {
                    trace!("Error {:?}", e);
                    break;
                }
            }
        }

        None
    }

    pub fn feed(&mut self, buf: &[u8]) {
        let _ = self.parser.source_mut().write(buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::str;
    #[test]
    fn test_extract() {
        let mut x = XmppParser::new();

        x.feed("<?xml version='1.0'?><stream:stream id='16243086933621190650' version='1.0' xml:lang='en' xmlns:stream='http://etherx.jabber.org/streams' from='exampl".as_bytes());
        match x.next_event() {
            None => assert!(true),
            Some(_) => assert!(false),
        }

        x.feed(b"e.com' xmlns='jabber:client'>");
        match x.next_event() {
            Some(_) => assert!(true),
            None => assert!(false),
        }

        x.feed(b"<stream:features><starttls xmlns='urn:ietf:params:xml:ns:xmpp-tls'><required/></starttls></stream:features>");
        x.feed(b"</stream:stream>");

        match x.next_event() {
            None => assert!(false),
            Some(_) => assert!(true),
        }
    }
}
