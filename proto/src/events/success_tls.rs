use events::{Event, ToEvent};
use events::NonStanzaEvent::SuccessTlsEvent;
use config::XMPPConfig;

#[derive(Debug, Clone, XmppEvent)]
#[non_stanza(event="SuccessTlsEvent(_)")]
pub struct SuccessTls {}

impl SuccessTls {
    pub fn new(_: &XMPPConfig) -> SuccessTls {
        SuccessTls {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct() {
        let x = SuccessTls {};
        let _ = x.clone();
    }
}
