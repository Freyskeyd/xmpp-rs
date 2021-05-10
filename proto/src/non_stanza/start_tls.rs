use crate::{NonStanza, Packet};

#[derive(Debug, Clone)]
pub struct StartTls {}

impl From<StartTls> for Packet {
    fn from(s: StartTls) -> Self {
        NonStanza::StartTls(s).into()
    }
}
