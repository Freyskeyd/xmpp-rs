use events::{Event, ToEvent};
use events::NonStanzaEvent::ProceedTlsEvent;

use config::XMPPConfig;

#[derive(Debug, Clone, XmppEvent)]
#[non_stanza(event="ProceedTlsEvent(_)")]
pub struct ProceedTls {}

impl ProceedTls {
    pub fn new(_: &XMPPConfig) -> ProceedTls {
        ProceedTls {}
    }
}
