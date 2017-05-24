use super::{Event, FromXmlElement, ToXmlElement, ToEvent};
use super::NonStanzaEvent::StreamFeaturesEvent;
use xmpp_config::XMPPConfig;
use xmpp_config::ns;
use xmpp_xml::Element;
use std::io;

/// Define possible features found in a stream:features
#[derive(Debug, Clone, PartialEq)]
pub enum Features {
    StartTlsInit,
    Bind,
    Mechanims(Vec<String>),
    Unknown,
}

#[derive(Debug, Clone, XmppEvent)]
#[non_stanza(event="StreamFeaturesEvent(_)")]
pub struct StreamFeatures {
    config: XMPPConfig,
    features: Features,
    session: Option<Element>,
}

impl StreamFeatures {
    pub fn new(config: &XMPPConfig) -> StreamFeatures {
        StreamFeatures {
            config: config.clone(),
            features: Features::Unknown,
            session: None,
        }
    }

    pub fn get_features(&self) -> Features {
        self.features.clone()
    }
}

impl ToXmlElement for StreamFeatures {
    type Error = io::Error;
    fn to_element(&self) -> Result<Element, io::Error> {
        let mut root = Element::new("stream:features");
        match self.features {
            Features::Bind => {
                root.append_new_child((ns::BIND, "bind"));
                if let Some(ref session) = self.session {
                    root.append_child(session.clone());
                }
            }
            Features::StartTlsInit => {
                root.append_new_child((ns::TLS, "starttls"))
                    .append_new_child((ns::TLS, "required"));
            }
            Features::Mechanims(ref vec_mecha) => {
                let mechs = root.append_new_child((ns::SASL, "mechanisms"));

                for mech in vec_mecha {
                    mechs
                        .append_new_child((ns::SASL, "mechanism"))
                        .set_text(mech.to_string());
                }
            }
            _ => {}
        }

        Ok(root)
    }
}

impl FromXmlElement for StreamFeatures {
    type Error = io::Error;
    fn from_element(e: Element) -> Result<StreamFeatures, Self::Error> {
        let mut features = Features::Unknown;
        let mut session = None;

        if e.find((ns::BIND, "bind")).is_some() {
            features = Features::Bind;
            if let Some(sess) = e.find((ns::SESSION, "session")) {
                session = Some(sess.clone());
            }
        }

        if let Some(starttls) = e.find((ns::TLS, "starttls")) {
            if starttls.find((ns::TLS, "required")).is_some() {
                features = Features::StartTlsInit;
            }
        }

        if let Some(mechanisms) = e.find((ns::SASL, "mechanisms")) {
            let mut mechanisms_list = Vec::new();
            for child in mechanisms.find_all((ns::SASL, "mechanism")) {
                mechanisms_list.push(child.text().to_string());
            }

            features = Features::Mechanims(mechanisms_list);
        }

        Ok(StreamFeatures {
               config: XMPPConfig::new(),
               features: features,
               session: session,
           })
    }
}
