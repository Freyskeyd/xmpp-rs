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

    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("Starting UnauthenticatedSession");
    }

    fn stopping(&mut self, _ctx: &mut Self::Context) -> actix::Running {
        println!("Stopping UnauthenticatedSession");
        actix::Running::Stop
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        println!("UnauthenticatedSession Stopped");
    }
}

impl Handler<SessionManagementPacketResult> for UnauthenticatedSession {
    type Result = ();

    fn handle(&mut self, packet: SessionManagementPacketResult, _ctx: &mut Self::Context) -> Self::Result {
        self.packets.push(packet);
    }
}
