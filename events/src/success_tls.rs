use super::{Event, ToEvent};
use super::NonStanzaEvent::SuccessTlsEvent;

#[derive(Debug, Clone, XmppEvent)]
#[non_stanza(event="SuccessTlsEvent(_)")]
pub struct SuccessTls {}

impl SuccessTls {
    pub fn new() -> SuccessTls {
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
