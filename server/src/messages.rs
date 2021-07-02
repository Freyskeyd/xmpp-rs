use std::{error::Error, fmt};

use crate::sessions::state::SessionState;
use actix::{Message, Recipient};
use tokio::sync::mpsc::Sender;
use xmpp_proto::{Auth, Packet};

pub(crate) mod system;
pub(crate) mod tcp;

#[derive(Debug, Message)]
#[rtype(result = "Result<(), ()>")]
pub(crate) struct SessionManagementPacket {
    pub(crate) session_state: SessionState,
    pub(crate) packet: Packet,
    pub(crate) referer: Sender<SessionManagementPacketResult>,
}

#[derive(Debug, Message)]
#[rtype(result = "Result<(), ()>")]
pub(crate) struct SessionPacket {
    pub(crate) packet: Packet,
    pub(crate) referer: Recipient<SessionManagementPacketResult>,
}

#[derive(Message, derive_builder::Builder, Debug, Clone)]
#[builder(setter(into))]
#[rtype("()")]
pub(crate) struct SessionManagementPacketResult {
    #[builder(default = "SessionState::Opening")]
    pub(crate) session_state: SessionState,
    #[builder(default = "Vec::new()", setter(each = "packet", into = "true"))]
    pub(crate) packets: Vec<Packet>,
}

impl SessionManagementPacketResult {
    pub(crate) fn send(self, referer: Option<Sender<Self>>) {
        if let Some(r) = referer {
            let _ = r.try_send(self);
        }
    }
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
        write!(f, "{}", "error")
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
#[rtype("()")]
pub struct AuthenticationRequest {
    pub(crate) packet: Auth,
}

impl AuthenticationRequest {
    pub(crate) fn new(packet: Auth) -> Self {
        Self { packet }
    }
}
