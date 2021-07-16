use crate::sessions::state::StaticSessionState;
use actix::Message;
use jid::Jid;
use std::{error::Error, fmt};
use xmpp_proto::{Auth, Packet};

pub(crate) mod system;
pub(crate) mod tcp;

#[derive(Debug, Message)]
#[rtype(result = "Result<(), ()>")]
pub(crate) struct SessionPacket {
    pub(crate) packet: Packet,
}

#[derive(Message, derive_builder::Builder, Debug, Clone)]
#[builder(setter(into, strip_option))]
#[rtype("()")]
pub(crate) struct SessionManagementPacketResult {
    #[builder(default = "Vec::new()", setter(each = "packet", into = "true"))]
    pub(crate) packets: Vec<Packet>,
    pub(crate) session_state: StaticSessionState,
}

#[derive(Debug)]
pub(crate) enum SessionManagementPacketError {
    Unknown,
}

impl Error for SessionManagementPacketError {
    fn description(&self) -> &str {
        "error"
    }
}

impl fmt::Display for SessionManagementPacketError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error")
    }
}

impl From<actix::MailboxError> for SessionManagementPacketError {
    fn from(_: actix::MailboxError) -> Self {
        Self::Unknown
    }
}

impl From<SessionManagementPacketResultBuilderError> for SessionManagementPacketError {
    fn from(_: SessionManagementPacketResultBuilderError) -> Self {
        Self::Unknown
    }
}

#[derive(Message)]
#[rtype("Result<Jid, ()>")]
pub struct AuthenticationRequest {
    pub(crate) packet: Auth,
}

impl AuthenticationRequest {
    pub(crate) fn new(packet: Auth) -> Self {
        Self { packet }
    }
}
