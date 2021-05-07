use xmpp_xml::Element;

use crate::FromXmlElement;

#[derive(Default, Debug, Clone)]
pub struct Auth {
    mechanism: Option<String>,
    challenge: Option<String>,
}

impl FromXmlElement for Auth {
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
        let proceed = Auth::from_element(element).unwrap();

        assert_eq!(proceed.mechanism.unwrap(), "PLAIN");
        assert_eq!(proceed.challenge.unwrap(), "AGp1bGlldAByMG0zMG15cjBtMzA=");
    }
}
