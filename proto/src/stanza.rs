use xmpp_xml::Element;

use crate::ToXmlElement;

pub use self::generic_iq::*;

mod generic_iq;

/// Define a sub part of a Packet, a Stanza is the representation of an Xmpp Stanza which can be a
/// Presence, an IQ or a Message.
#[derive(Debug, Clone)]
pub enum Stanza {
    IQ(GenericIq),
    Message(Element),
    Presence(Element),
}

impl ToXmlElement for Stanza {
    type Error = std::io::Error;

    fn to_element(&self) -> Result<Element, Self::Error> {
        match self {
            Stanza::IQ(iq) => iq.to_element(),
            Stanza::Message(s) => s.to_element(),
            Stanza::Presence(s) => s.to_element(),
        }
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::FromXmlElement;

    use super::*;

    #[test]
    fn to_element() {
        let mut element = Element::new("iq");
        element.set_attr("id", GenericIq::unique_id()).set_attr("type", "get");
        element.append_child(Element::new("test"));
        let iq = GenericIq::from_element(&element).unwrap();

        assert!(matches!(Stanza::IQ(iq).to_element(), Ok(_)));
        assert!(matches!(Stanza::Message(Element::new("message")).to_element(), Ok(_)));
        assert!(matches!(Stanza::Presence(Element::new("presence")).to_element(), Ok(_)));
    }
}
