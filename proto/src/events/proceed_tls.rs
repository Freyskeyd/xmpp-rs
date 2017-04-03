use std::str::FromStr;
use std::string::ParseError;
use events::XMPPConfig;

#[derive(Debug, Clone)]
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
