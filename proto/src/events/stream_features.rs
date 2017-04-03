use std::str::FromStr;
use std::string::ParseError;
use ns;
use events::XMPPConfig;

#[derive(Debug, Clone)]
pub struct StreamFeatures {
    config: XMPPConfig,
}

impl StreamFeatures {
    pub fn new(config: &XMPPConfig) -> StreamFeatures {
        StreamFeatures {
            config: config.clone(),
        }
    }
}

impl FromStr for StreamFeatures {
    type Err = ParseError;
    fn from_str(_: &str) -> Result<Self, Self::Err> {
        Ok(StreamFeatures {
            config: XMPPConfig::new(),
        })
    }
}

impl ToString for StreamFeatures {
    fn to_string(&self) -> String {
        format!("<starttls xmlns='{ns}' />",ns=ns::TLS)
    }
}
