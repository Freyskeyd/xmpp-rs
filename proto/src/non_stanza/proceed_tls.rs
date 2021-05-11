use xmpp_xml::Element;

use crate::{ns, FromXmlElement, NonStanza, Packet, ToXmlElement};

#[derive(Default, Debug, Clone)]
pub struct ProceedTls {
    mechanism: Option<String>,
    challenge: Option<String>,
}

impl From<ProceedTls> for Packet {
    fn from(s: ProceedTls) -> Self {
        NonStanza::ProceedTls(s).into()
    }
}

impl ToXmlElement for ProceedTls {
    type Error = std::io::Error;
    fn to_element(&self) -> Result<Element, std::io::Error> {
        let root = Element::new((ns::TLS, "proceed"));

        Ok(root)
    }
}

impl FromXmlElement for ProceedTls {
    type Error = std::io::Error;
    fn from_element(e: Element) -> Result<Self, Self::Error> {
        let p = Self {
            mechanism: e.get_attr("mechanism").map(|mechanism| mechanism.to_string()),
            challenge: Some(e.text().to_string()),
        };

        Ok(p)
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use circular::Buffer;
    use xmpp_xml::{
        xml::{reader::XmlEvent, ParserConfig},
        WriteOptions,
    };

    use super::*;

    const EXPECTED_PROCEEDTLS: &'static str = r#"<proceed xmlns="urn:ietf:params:xml:ns:xmpp-tls" />"#;

    #[test]
    fn to_element() {
        let proceed = ProceedTls::default();

        let mut output: Vec<u8> = Vec::new();
        let _ = proceed.to_element().unwrap().to_writer_with_options(&mut output, WriteOptions::new().set_xml_prolog(None));

        let generated = String::from_utf8(output).unwrap();

        assert!(EXPECTED_PROCEEDTLS == generated);
    }

    #[test]
    fn from() {
        let mut cfg = ParserConfig::new().whitespace_to_characters(true);
        cfg.ignore_end_of_stream = true;
        let mut reader = cfg.create_reader(Buffer::with_capacity(4096));

        reader.source_mut().write(EXPECTED_PROCEEDTLS.as_bytes()).unwrap();
        let _ = reader.next().unwrap();
        let x = reader.next().unwrap();

        assert!(matches!(x, XmlEvent::StartElement { .. }));

        if let XmlEvent::StartElement { name, attributes, namespace } = x {
            let packet = Packet::parse(&mut reader, name, namespace, attributes);
            assert!(
                matches!(packet, Ok(Packet::NonStanza(ref stanza)) if matches!(**stanza, NonStanza::ProceedTls(_))),
                "Packet wasn't an ProceedTls, it was: {:?}",
                packet
            );
        }
    }
}
