use std::str::FromStr;
use std::string::ParseError;
use jid::Jid;
use events::XMPPConfig;

#[derive(Debug, Clone)]
pub struct Presence {
    config: XMPPConfig,
    jid: Jid
}

impl Presence {
    pub fn new(config: &XMPPConfig, jid: Jid) -> Presence {
        Presence {
            config: config.clone(),
            jid: jid
        }
    }
}

impl FromStr for Presence {
    type Err = ParseError;
    fn from_str(_: &str) -> Result<Self, Self::Err> {
        Ok(Presence {
            config: XMPPConfig::new(),
            jid: Jid::from_str("")?
        })
    }
}

impl ToString for Presence {
    fn to_string(&self) -> String {
        format!("<presence from='{to}' />", to=self.jid)
    }
}
