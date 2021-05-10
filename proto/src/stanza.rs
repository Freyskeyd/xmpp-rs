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
