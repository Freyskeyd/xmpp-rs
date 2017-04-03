use std::str::FromStr;
use std::string::ParseError;
use events::XMPPConfig;

#[derive(Debug, Clone)]
pub struct SuccessTls {
}

impl SuccessTls {
    pub fn new(_: &XMPPConfig) -> SuccessTls {
        SuccessTls {
        }
    }
}

impl FromStr for SuccessTls {
    type Err = ParseError;
    fn from_str(_: &str) -> Result<Self, Self::Err> {
        Ok(SuccessTls {
        })
    }
}

impl ToString for SuccessTls {
    fn to_string(&self) -> String {
        String::new()
    }
}
