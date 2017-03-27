use std::result::Result;
use std::str::FromStr;
use std::string::ParseError;

#[derive(Clone, Debug, PartialEq)]
pub struct Jid {
    node: Option<String>,
    domain: String,
    resource: Option<String>
}

impl FromStr for Jid {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, ParseError> {
        Ok(Jid {
            node: None,
            domain: input.to_string(),
            resource: None
        })
    }
}
