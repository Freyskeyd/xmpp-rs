use events::{Event, ToEvent};
use events::NonStanzaEvent::StartTlsEvent;

use events::ToXmlElement;
use std::io;
use ns;
use xmpp_xml::Element;
use config::XMPPConfig;

#[derive(Debug, Clone, XmppEvent)]
#[non_stanza(event="StartTlsEvent(_)")]
pub struct StartTls {
    config: XMPPConfig,
}

impl StartTls {
    pub fn new(config: &XMPPConfig) -> StartTls {
        StartTls { config: config.clone() }
    }
}

impl ToXmlElement for StartTls {
    type Error = io::Error;
    fn to_element(&self) -> Result<Element, Self::Error> {
        Ok(Element::new((ns::TLS, "starttls")))
    }
}
