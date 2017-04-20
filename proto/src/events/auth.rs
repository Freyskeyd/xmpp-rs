use std::str::FromStr;
use std::string::ParseError;
use events::NonStanzaEvent::AuthEvent;
use events::Event;
use events::EventTrait;
use config::XMPPConfig;
use credentials::Credentials;
use base64::encode;
use std::str;

#[derive(Debug, Clone, XmppEvent)]
#[non_stanza(event = "AuthEvent(_)")]
pub struct Auth {
    config: XMPPConfig,
    credentials: Credentials
}

impl Auth {
    pub fn new(config: &XMPPConfig, credentials: Credentials) -> Auth {
        Auth {
            config: config.clone(),
            credentials: credentials
        }
    }
}

impl FromStr for Auth {
    type Err = ParseError;
    fn from_str(_: &str) -> Result<Self, Self::Err> {
        Ok(Auth {
            config: XMPPConfig::new(),
            credentials: Credentials { ..Credentials::default() }
        })
    }
}

impl ToString for Auth {
    fn to_string(&self) -> String {
        let mut data: Vec<u8> = Vec::new();
        data.push(0);
        let creds = format!("{}", self.credentials.jid);
        data.extend(creds.as_bytes());
        data.push(0);
        data.extend(self.credentials.password.as_bytes());

        let bytes = str::from_utf8(&data).unwrap().as_bytes();
        let plain = encode(bytes);
        format!("<auth xmlns='urn:ietf:params:xml:ns:xmpp-sasl' mechanism='PLAIN'>{}</auth>", plain)
    }
}
