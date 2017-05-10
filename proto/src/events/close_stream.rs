use super::Event;
use super::NonStanzaEvent;
use super::EventTrait;

#[derive(Debug, Clone, XmppEvent)]
#[non_stanza(event = "NonStanzaEvent::CloseStreamEvent")]
pub struct CloseStream {}

impl Default for CloseStream {
    fn default() -> Self {
        Self::new()
    }
}

impl CloseStream {
    pub fn new() -> CloseStream {
        CloseStream {}
    }
}
