use uuid::Uuid;
use xmpp_xml::{Element, QName};

use crate::{ns, ToXmlElement};

#[derive(derive_builder::Builder, Debug, Clone)]
#[builder(setter(into))]
pub struct OpenStream {
    pub id: Uuid,
    pub lang: String,
    pub version: String,
    #[builder(setter(into, strip_option), default)]
    pub to: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub from: Option<String>,
    pub xmlns: String,
}

impl ToXmlElement for OpenStream {
    type Error = std::io::Error;
    fn to_element(&self) -> Result<Element, Self::Error> {
        let mut element = Element::new((ns::CLIENT, "stream")).write_end_tag(false);

        let _ = element.set_namespace_prefix(ns::STREAM, "stream");

        let qname_stream = QName::from_ns_name(Some(ns::STREAM), "stream");
        element.set_tag(&qname_stream);
        element.set_attr("version", self.version.to_string());
        element.set_attr("lang", self.lang.to_string());
        element.set_attr("id", self.id.to_string());

        if let Some(ref from) = self.from {
            element.set_attr("from", from.to_string());
        }

        if let Some(ref to) = self.to {
            element.set_attr("to", to.to_string());
        }

        Ok(element)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use circular::Buffer;
    use std::io::Write;
    use std::{convert::TryFrom, io::Cursor};

    use xmpp_xml::xml::{common::Position, reader::XmlEvent, EventReader, ParserConfig};
    use xmpp_xml::Element;

    #[test]
    fn from() {
        let initial_stream = "<stream:stream version='1.0' xmlns:stream='http://etherx.jabber.org/streams' to='example.com' xmlns='jabber:client'>";

        let mut cfg = ParserConfig::new().whitespace_to_characters(true);
        cfg.ignore_end_of_stream = true;
        let mut reader = cfg.create_reader(Buffer::with_capacity(4096));

        reader.source_mut().write(initial_stream.as_bytes()).unwrap();
        let x = reader.next().unwrap();
        let x = reader.next().unwrap();

        match x {
            XmlEvent::StartElement { name, attributes, namespace } => {
                let el = Element::from_start_element(name, attributes, namespace, None, &mut reader);
                println!("{:?}", el);
            }
            _ => assert!(false),
        }
    }
}
