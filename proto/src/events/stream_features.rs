use events::{Event, EventTrait};
use events::NonStanzaEvent::StreamFeaturesEvent;
use std::str::FromStr;
use std::string::ParseError;
use config::XMPPConfig;
use ns;
use elementtree::{WriteOptions, Element};

#[derive(Debug, Clone)]
enum Features {
    StartTlsInit,
    Bind,
    Mechanims(Vec<String>),
    Unknown
}

#[derive(Debug, Clone, XmppEvent)]
#[non_stanza(event="StreamFeaturesEvent(_)")]
pub struct StreamFeatures {
    config: XMPPConfig,
    features: Features,
    session: Option<Element>
}

impl StreamFeatures {
    pub fn new(config: &XMPPConfig) -> StreamFeatures {
        StreamFeatures {
            config: config.clone(),
            features: Features::Unknown,
            session: None
        }
    }

    pub fn from_element(e: Element) -> StreamFeatures {
        let mut features = Features::Unknown;
        let mut session = None;

        if let Some(_) = e.find((ns::BIND, "bind")) {
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

        StreamFeatures {
            config: XMPPConfig::new(),
            features: features,
            session: session
        }
    }
}

impl FromStr for StreamFeatures {
    type Err = ParseError;
    fn from_str(_: &str) -> Result<Self, Self::Err> {
        Ok(StreamFeatures {
            config: XMPPConfig::new(),
            features: Features::Unknown,
            session: None
        })
    }
}

impl ToString for StreamFeatures {
    fn to_string(&self) -> String {
        let mut out:Vec<u8> = Vec::new();
        let mut root = Element::new("stream:features");
        let options = WriteOptions::new()
            .set_xml_prolog(None);

        match self.features {
            Features::Bind => {
                root.append_new_child((ns::BIND, "bind"));
                if let Some(ref session) = self.session {
                    root.append_child(session.clone());
                }
            },
            Features::StartTlsInit => {
                root.append_new_child((ns::TLS, "starttls")).append_new_child((ns::TLS, "required"));
            },
            Features::Mechanims(ref vec_mecha) => {
                let mechs = root.append_new_child((ns::SASL, "mechanisms"));

                for mech in vec_mecha {
                    mechs.append_new_child((ns::SASL, "mechanism")).set_text(mech.to_string());
                }
            }
            _ => {}
        };

        root.to_writer_with_options(&mut out, options).unwrap();
        String::from_utf8(out).unwrap()
    }
}
