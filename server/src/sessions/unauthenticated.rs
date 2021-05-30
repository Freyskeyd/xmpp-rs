use crate::sessions::{state::SessionState, GetPacket, SessionManagementPacketResult};
use actix::{Actor, Context, Handler};
use xmpp_proto::Packet;

#[derive(Default)]
pub(crate) struct UnauthenticatedSession {
    pub(crate) state: SessionState,
    pub(crate) packets: Vec<SessionManagementPacketResult>,
}

impl Actor for UnauthenticatedSession {
    type Context = Context<Self>;
}

impl Handler<SessionManagementPacketResult> for UnauthenticatedSession {
    type Result = ();

    fn handle(&mut self, packet: SessionManagementPacketResult, _ctx: &mut Self::Context) -> Self::Result {
        self.packets.push(packet);
        ()
    }
}

impl Handler<GetPacket> for UnauthenticatedSession {
    type Result = Result<Vec<Packet>, ()>;

    fn handle(&mut self, _msg: GetPacket, _ctx: &mut Self::Context) -> Self::Result {
        Ok(vec![])
    }
}
