use crate::Packet;
use crate::ToXmlElement;
use crate::{ns, NonStanza};
use xmpp_xml::Element;

#[derive(Debug, Clone)]
pub struct StreamError {
    pub kind: StreamErrorKind,
    //TODO: Implement error text
    // pub text: String
}

#[derive(Debug, Clone)]
pub enum StreamErrorKind {
    BadNamespacePrefix,
    HostUnknown,
    InvalidNamespace,
    NotAuthorized,
    UnsupportedEncoding,
    UnsupportedVersion,
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
            StreamErrorKind::BadNamespacePrefix => root.append_new_child(("urn:ietf:params:xml:ns:xmpp-streams", "bad-namespace-prefix")),
            StreamErrorKind::HostUnknown => root.append_new_child(("urn:ietf:params:xml:ns:xmpp-streams", "host-unknown")),
            StreamErrorKind::InvalidNamespace => root.append_new_child(("urn:ietf:params:xml:ns:xmpp-streams", "invalid-namespace")),
            StreamErrorKind::NotAuthorized => root.append_new_child(("urn:ietf:params:xml:ns:xmpp-streams", "not-authorized")),
            StreamErrorKind::UnsupportedEncoding => root.append_new_child(("urn:ietf:params:xml:ns:xmpp-streams", "unsupported-encoding")),
            StreamErrorKind::UnsupportedVersion => root.append_new_child(("urn:ietf:params:xml:ns:xmpp-streams", "unsupported-version")),
        };

        Ok(root)
    }
}
