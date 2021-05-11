use xmpp_xml::Element;

use crate::{FromXmlElement, NonStanza, Packet, PacketParsingError};

#[derive(Debug, Clone)]
pub struct StartTls {}

impl From<StartTls> for Packet {
    fn from(s: StartTls) -> Self {
        NonStanza::StartTls(s).into()
    }
}

impl FromXmlElement for StartTls {
    type Error = PacketParsingError;

    fn from_element(_: Element) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        Ok(Self {})
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use circular::Buffer;
    use xmpp_xml::xml::{reader::XmlEvent, ParserConfig};

    use super::*;

    const EXPECTED_STARTTLS: &'static str = "<starttls xmlns='urn:ietf:params:xml:ns:xmpp-tls'/>";

    #[test]
    fn from() {
        let mut cfg = ParserConfig::new().whitespace_to_characters(true);
        cfg.ignore_end_of_stream = true;
        let mut reader = cfg.create_reader(Buffer::with_capacity(4096));

        reader.source_mut().write(EXPECTED_STARTTLS.as_bytes()).unwrap();
        let _ = reader.next().unwrap();
        let x = reader.next().unwrap();

        assert!(matches!(x, XmlEvent::StartElement { .. }));

        if let XmlEvent::StartElement { name, attributes, namespace } = x {
            let packet = Packet::parse(&mut reader, name, namespace, attributes);
            assert!(
                matches!(packet, Ok(Packet::NonStanza(ref stanza)) if matches!(**stanza, NonStanza::StartTls(_))),
                "Packet wasn't an StartTls, it was: {:?}",
                packet
            );
        }
    }
}
