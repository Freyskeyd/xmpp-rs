use xmpp_xml::Element;

use crate::{ns, NonStanza, Packet, ToXmlElement};

#[derive(Default, Debug, Clone)]
pub struct SASLSuccess {}

impl From<SASLSuccess> for Packet {
    fn from(s: SASLSuccess) -> Self {
        NonStanza::SASLSuccess(s).into()
    }
}

impl ToXmlElement for SASLSuccess {
    type Error = std::io::Error;
    fn to_element(&self) -> Result<Element, Self::Error> {
        Ok(Element::new((ns::SASL, "success")))
    }
}

#[cfg(test)]
mod tests {
    use xmpp_xml::WriteOptions;

    use super::*;

    const EXPECTED_SUCCESS: &'static str = r#"<success xmlns="urn:ietf:params:xml:ns:xmpp-sasl" />"#;

    #[test]
    fn from() {
        let packet: Packet = SASLSuccess::default().into();
        assert!(
            matches!(packet, Packet::NonStanza(ref stanza) if matches!(**stanza, NonStanza::SASLSuccess(_))),
            "Packet wasn't an SASLSuccess",
        );
    }

    #[test]
    fn to() {
        let mut output: Vec<u8> = Vec::new();
        let _ = SASLSuccess::default()
            .to_element()
            .unwrap()
            .to_writer_with_options(&mut output, WriteOptions::new().set_xml_prolog(None));

        let generated = String::from_utf8(output).unwrap();

        assert!(EXPECTED_SUCCESS == generated);
    }
}
