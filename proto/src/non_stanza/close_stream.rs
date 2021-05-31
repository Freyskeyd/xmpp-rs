use crate::Packet;
use crate::ToXmlElement;
use crate::{ns, NonStanza};
use xmpp_xml::Element;

#[derive(Debug, Clone)]
pub struct CloseStream {}

impl From<CloseStream> for Packet {
    fn from(s: CloseStream) -> Self {
        NonStanza::CloseStream(s).into()
    }
}

impl ToXmlElement for CloseStream {
    type Error = std::io::Error;
    fn to_element(&self) -> Result<Element, std::io::Error> {
        let root = Element::new((ns::STREAM, "stream"));

        Ok(root)
    }
}
