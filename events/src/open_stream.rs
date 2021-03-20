use super::Event;
use super::NonStanzaEvent;
use super::ToEvent;
use super::ToXmlElement;
use std::io;
use xmpp_config::ns;
use xmpp_config::XMPPConfig;
use xmpp_xml::Element;
use xmpp_xml::QName;

#[derive(Debug, Clone, XmppEvent)]
#[non_stanza(event = "NonStanzaEvent::OpenStreamEvent(_)")]
pub struct OpenStream {
    config: XMPPConfig,
    pub to: Option<String>,
    pub xmlns: String,
}

impl OpenStream {
    pub fn new(config: &XMPPConfig) -> OpenStream {
        OpenStream {
            config: config.clone(),
            to: Some(config.get_domain().to_string()),
            xmlns: ns::CLIENT.to_string(),
        }
    }
}

impl ToXmlElement for OpenStream {
    type Error = io::Error;
    fn to_element(&self) -> Result<Element, Self::Error> {
        let mut element = Element::new((ns::CLIENT, "stream"));
        let _ = element.set_namespace_prefix(ns::STREAM, "stream");

        let qname_stream = QName::from_ns_name(Some(ns::STREAM), "stream");
        element.set_tag(&qname_stream);
        element.set_attr("version", "1.0");
        if let Some(ref t) = self.to {
            element.set_attr("to", t.to_string());
        }

        Ok(element)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use xmpp_config::XMPPConfig;

    #[test]
    fn compile() {
        let o = OpenStream::new(&XMPPConfig::new().set_domain("hey"));

        let e = o.to_event();

        assert!(!e.is_message());
        assert!(!e.is_iq());
        assert!(e.is_non_stanza());

        // assert!(e.is::)
        match o.to_event() {
            Event::NonStanza(non_stanza) => match *non_stanza {
                NonStanzaEvent::OpenStreamEvent(_) => {
                    assert!(true);
                }
                _ => panic!(""),
            },
            _ => panic!(""),
        }
    }
}
