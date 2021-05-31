use crate::Packet;
use crate::ToXmlElement;
use crate::{ns, NonStanza};
use xmpp_xml::Element;

#[derive(Debug, Clone)]
pub struct StreamError {
    pub kind: StreamErrorKind,
}

#[derive(Debug, Clone)]
pub enum StreamErrorKind {
    HostUnknown,
}

impl From<StreamError> for Packet {
    fn from(s: StreamError) -> Self {
        NonStanza::StreamError(s).into()
    }
}

impl ToXmlElement for StreamError {
    type Error = std::io::Error;
    fn to_element(&self) -> Result<Element, std::io::Error> {
        let mut root = Element::new((ns::STREAM, "error"));

        match self.kind {
            StreamErrorKind::HostUnknown => root.append_new_child(("urn:ietf:params:xml:ns:xmpp-streams", "host-unknown")),
        };

        Ok(root)
    }
}
