use events::{Event, EventTrait};
use events::NonStanzaEvent::ProceedTlsEvent;

use std::str::FromStr;
use std::string::ParseError;
use config::XMPPConfig;

#[derive(Debug, Clone, XmppEvent)]
#[non_stanza(event="ProceedTlsEvent(_)")]
pub struct ProceedTls {}

impl ProceedTls {
    pub fn new(_: &XMPPConfig) -> ProceedTls {
        ProceedTls {}
    }
}

impl FromStr for ProceedTls {
    type Err = ParseError;
    fn from_str(_: &str) -> Result<Self, Self::Err> {
        Ok(ProceedTls {  })
    }
}

impl ToString for ProceedTls {
    fn to_string(&self) -> String {
        String::new()
    }
}
