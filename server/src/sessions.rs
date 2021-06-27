use actix::prelude::*;
use actix_codec::AsyncWrite;
use tokio::sync::mpsc::Sender;
use xmpp_proto::Packet;

pub(crate) mod manager;
pub(crate) mod state;
pub(crate) mod unauthenticated;

/// Hold a session on a node
#[allow(dead_code)]
pub struct Session {
    #[allow(dead_code)]
    sink: Box<dyn AsyncWrite>,
}

impl Session {}

#[derive(Debug, Message)]
#[rtype(result = "Result<(), ()>")]
pub(crate) struct SessionManagementPacket {
    pub(crate) session_state: state::SessionState,
    pub(crate) packet: Packet,
    pub(crate) referer: Sender<SessionManagementPacketResult>,
}

#[derive(Message, derive_builder::Builder, Debug, Clone)]
#[builder(setter(into))]
#[rtype("()")]
pub(crate) struct SessionManagementPacketResult {
    #[builder(default = "state::SessionState::Opening")]
    pub(crate) session_state: state::SessionState,
    #[builder(default = "Vec::new()", setter(each = "packet", into = "true"))]
    pub(crate) packets: Vec<Packet>,
}

impl SessionManagementPacketResult {
    pub(crate) fn send(self, referer: Option<Sender<Self>>) {
        match referer {
            Some(r) => {
                let _ = r.try_send(self);
            }
            None => {}
        }
    }
}

#[derive(Message)]
#[rtype(result = "Result<Vec<Packet>, ()>")]
struct GetPacket {}
