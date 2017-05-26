use super::{Event, ToEvent};
use super::NonStanzaEvent::StartTlsEvent;

use super::ToXmlElement;
use std::io;
use xmpp_config::ns;
use xmpp_xml::Element;

#[derive(Debug, Clone, XmppEvent)]
#[non_stanza(event="StartTlsEvent(_)")]
pub struct StartTls {
}

impl StartTls {
    pub fn new() -> StartTls {
        StartTls { }
    }
}

impl ToXmlElement for StartTls {
    type Error = io::Error;
    fn to_element(&self) -> Result<Element, Self::Error> {
        Ok(Element::new((ns::TLS, "starttls")))
    }
}
