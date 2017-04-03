use std::str::FromStr;
use std::string::ParseError;
use ns;
use events::XMPPConfig;

#[derive(Debug, Clone)]
pub struct StartTls {
    config: XMPPConfig,
}

impl StartTls {
    pub fn new(config: &XMPPConfig) -> StartTls {
        StartTls {
            config: config.clone(),
        }
    }
}

impl FromStr for StartTls {
    type Err = ParseError;
    fn from_str(_: &str) -> Result<Self, Self::Err> {
        Ok(StartTls {
            config: XMPPConfig::new(),
        })
    }
}

impl ToString for StartTls {
    fn to_string(&self) -> String {
        format!("<starttls xmlns='{ns}' />",ns=ns::TLS)
    }
}
