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
    use super::*;

    #[test]
    fn parse_proceed_plain() {
        let element = Element::from_reader("<auth xmlns='urn:ietf:params:xml:ns:xmpp-sasl' mechanism='PLAIN'>AGp1bGlldAByMG0zMG15cjBtMzA=</auth>".as_bytes()).unwrap();
        let proceed = ProceedTls::from_element(element).unwrap();

        assert_eq!(proceed.mechanism.unwrap(), "PLAIN");
        assert_eq!(proceed.challenge.unwrap(), "AGp1bGlldAByMG0zMG15cjBtMzA=");
    }
}
