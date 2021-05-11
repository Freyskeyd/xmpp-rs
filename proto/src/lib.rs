mod non_stanza;
pub mod ns;
mod packet;
mod stanza;

use jid::Jid;

pub use non_stanza::*;
pub use packet::*;
pub use stanza::*;

use xmpp_xml::Element;

pub trait NonStanzaTrait {}

impl ToXmlElement for Element {
    type Error = std::io::Error;

    fn to_element(&self) -> Result<Element, Self::Error> {
        Ok(self.clone())
    }
}

/// FromXmlElement is used to transform an Element to an object
pub trait FromXmlElement {
    type Error;
    fn from_element(e: Element) -> Result<Self, Self::Error>
    where
        Self: Sized;
}

pub trait ToXmlElement {
    type Error;

    fn to_element(&self) -> Result<Element, Self::Error>;
}
