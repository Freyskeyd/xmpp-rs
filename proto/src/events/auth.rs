use events::NonStanzaEvent::AuthEvent;
use events::Event;
use events::EventTrait;
use events::ToXmlElement;
use events::FromXmlElement;
use config::XMPPConfig;
use credentials::Credentials;
use base64::encode;
use elementtree::Element;
use ns;
use sasl::client::Mechanism;
use sasl::client::mechanisms::Plain;
use std::io;

#[derive(Debug, Clone, XmppEvent)]
#[non_stanza(event = "AuthEvent(_)")]
pub struct Auth {
    config: XMPPConfig,
    credentials: Credentials,
}

impl Auth {
    pub fn new(config: &XMPPConfig, credentials: Credentials) -> Auth {
        Auth {
            config: config.clone(),
            credentials: credentials,
        }
    }
}

impl FromXmlElement for Auth {
    type Error = io::Error;
    fn from_element(_: Element) -> Result<Auth, Self::Error> {
        Ok(Auth {
            config: XMPPConfig::new(),
            credentials: Credentials { ..Credentials::default() },
        })
    }
}

impl ToXmlElement for Auth {
    type Error = io::Error;
    fn to_element(&self) -> Result<Element, Self::Error> {
        let mut element = Element::new((ns::SASL, "auth"));
        let creds = self.credentials.format();
        let mut mecanism = Plain::from_credentials(creds).unwrap();
        let bytes = mecanism.initial().unwrap();
        let plain = encode(&bytes);

        element.set_attr("mechanism", "PLAIN");
        element.set_text(plain);

        Ok(element)
    }
}
