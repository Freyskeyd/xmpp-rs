use super::{Event, ToEvent};
use super::NonStanzaEvent::ProceedTlsEvent;


#[derive(Debug, Clone, XmppEvent)]
#[non_stanza(event="ProceedTlsEvent(_)")]
pub struct ProceedTls {}

impl ProceedTls {
    pub fn new() -> ProceedTls {
        ProceedTls {}
    }
}
