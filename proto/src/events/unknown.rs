use std::str::FromStr;
use std::string::ParseError;
use events::XMPPConfig;


#[derive(Debug, Clone)]
pub struct Unknown {}

impl Unknown {
    pub fn new(_: &XMPPConfig) -> Unknown { Unknown {} }
}

impl FromStr for Unknown {
    type Err = ParseError;
    fn from_str(_: &str) -> Result<Self, Self::Err> { Ok(Unknown {}) }
}

impl ToString for Unknown {
    fn to_string(&self) -> String { String::new() }
}
