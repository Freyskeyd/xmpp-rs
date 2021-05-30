use xmpp_xml::Element;

use crate::{FromXmlElement, NonStanza, Packet};

#[derive(Default, Debug, Clone)]
pub struct Auth {
    mechanism: Option<String>,
    challenge: Option<String>,
}

impl Auth {
    /// Get a reference to the auth's challenge.
    pub fn challenge(&self) -> &Option<String> {
        &self.challenge
    }

    /// Get a reference to the auth's mechanism.
    pub fn mechanism(&self) -> Option<&str> {
        self.mechanism.as_ref().map(|v| v.as_ref())
    }
}

impl From<Auth> for Packet {
    fn from(s: Auth) -> Self {
        NonStanza::Auth(s).into()
    }
}

impl FromXmlElement for Auth {
    type Error = std::io::Error;
    fn from_element(e: &Element) -> Result<Self, Self::Error> {
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
    use xmpp_xml::xml::{reader::XmlEvent, ParserConfig};

    use super::*;

    const EXPECTED_AUTH: &'static str = "<auth xmlns='urn:ietf:params:xml:ns:xmpp-sasl' mechanism='PLAIN'>AGp1bGlldAByMG0zMG15cjBtMzA=</auth>";

    #[test]
    fn parse_proceed_plain() {
        let element = Element::from_reader(EXPECTED_AUTH.as_bytes()).unwrap();
        let proceed = Auth::from_element(element).unwrap();

        assert_eq!(proceed.mechanism.unwrap(), "PLAIN");
        assert_eq!(proceed.challenge.unwrap(), "AGp1bGlldAByMG0zMG15cjBtMzA=");
    }

    #[test]
    fn from() {
        let mut cfg = ParserConfig::new().whitespace_to_characters(true);
        cfg.ignore_end_of_stream = true;
        let mut reader = cfg.create_reader(Buffer::with_capacity(4096));

        reader.source_mut().write(EXPECTED_AUTH.as_bytes()).unwrap();
        let _ = reader.next().unwrap();
        let x = reader.next().unwrap();

        assert!(matches!(x, XmlEvent::StartElement { .. }));

        if let XmlEvent::StartElement { name, attributes, namespace } = x {
            let packet = Packet::parse(&mut reader, name, namespace, attributes);
            assert!(
                matches!(packet, Ok(Packet::NonStanza(ref stanza)) if matches!(**stanza, NonStanza::Auth(_))),
                "Packet wasn't an Auth, it was: {:?}",
                packet
            );
        }
    }
}
