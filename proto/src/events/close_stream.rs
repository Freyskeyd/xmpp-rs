use std::str::FromStr;
use super::Event;
use super::NonStanzaEvent;
use super::EventTrait;
use std::string::ParseError;

#[derive(Debug, Clone, XmppEvent)]
#[non_stanza(event = "NonStanzaEvent::CloseStreamEvent")]
pub struct CloseStream {
}

impl CloseStream {
    pub fn new() -> CloseStream {
        CloseStream {
        }
    }
}

impl FromStr for CloseStream {
    type Err = ParseError;
    fn from_str(_: &str) -> Result<Self, Self::Err> {
        Ok(CloseStream {})
    }
}

impl ToString for CloseStream {
    fn to_string(&self) -> String {
        format!("</stream:stream>")
    }
}
