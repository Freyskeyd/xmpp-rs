use actix::prelude::*;
use log::trace;
use tokio::sync::mpsc::Sender;
use xmpp_proto::{Auth, SASLSuccess};

use crate::sessions::{state::SessionState, SessionManagementPacketResult, SessionManagementPacketResultBuilder};

#[derive(Default)]
pub struct AuthenticationManager {}

impl Supervised for AuthenticationManager {}

impl SystemService for AuthenticationManager {}
impl Actor for AuthenticationManager {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        trace!("AuthenticationManager started");
    }
}

impl Handler<AuthenticationRequest> for AuthenticationManager {
    type Result = ();

    fn handle(&mut self, msg: AuthenticationRequest, _ctx: &mut Self::Context) -> Self::Result {
        if let Some("PLAIN") = msg.packet.mechanism() {
            let mut response = SessionManagementPacketResultBuilder::default();
            response.session_state(SessionState::Authenticated).packet(SASLSuccess::default().into());

            if let Ok(res) = response.build() {
                res.send(msg.referer)
            }
        }
    }
}

#[derive(Message)]
#[rtype("()")]
pub struct AuthenticationRequest {
    packet: Auth,
    referer: Sender<SessionManagementPacketResult>,
}

impl AuthenticationRequest {
    pub(crate) fn new(packet: Auth, referer: Sender<SessionManagementPacketResult>) -> Self {
        Self { packet, referer }
    }
}
