use xmpp_xml::Element;

use crate::{ns, NonStanza, Packet, ToXmlElement};

#[derive(Debug, Clone)]
pub struct StreamFeatures {
    pub features: Features,
}

impl From<StreamFeatures> for Packet {
    fn from(s: StreamFeatures) -> Self {
        NonStanza::StreamFeatures(s).into()
    }
}

impl ToXmlElement for StreamFeatures {
    type Error = std::io::Error;
    fn to_element(&self) -> Result<Element, std::io::Error> {
        let mut root = Element::new("stream:features");

        match self.features {
            Features::StartTlsInit => {
                let starttls = root.append_new_child((ns::TLS, "starttls"));
                starttls.append_new_child((ns::TLS, "required"));
            }
            Features::Bind => {
                root.append_new_child((ns::BIND, "bind"));
            }
            Features::Mechanisms(ref mechanisms) => {
                let node = root.append_new_child((ns::SASL, "mechanisms"));
                mechanisms.iter().for_each(|mech| {
                    node.append_new_child((ns::SASL, "mechanism")).set_text(mech);
                });
            }
            Features::Unknown => {}
        }

        Ok(root)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Features {
    StartTlsInit,
    Bind,
    Mechanisms(Vec<String>),
    Unknown,
}
