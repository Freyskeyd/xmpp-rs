use std::collections::HashMap;

use actix::prelude::*;
use log::trace;

use xmpp_proto::Auth;

use crate::CONFIG;

type Vhost = String;

pub struct AuthenticationManager {
    authenticators: HashMap<Vhost, Vec<Recipient<AuthenticationRequest>>>,
}

impl Default for AuthenticationManager {
    fn default() -> Self {
        Self { authenticators: HashMap::new() }
    }
}
impl AuthenticationManager {
    pub(crate) fn register(mut self, authenticators: &HashMap<String, Recipient<AuthenticationRequest>>) -> Self {
        CONFIG.vhosts.iter().for_each(|(vhost, config)| {
            config.authenticators.iter().for_each(|authenticator| match authenticator.as_ref() {
                "memory" => {}
                custom => {
                    if let Some(recipient) = authenticators.get(custom) {
                        self.authenticators.entry(vhost.clone()).or_insert(Vec::new()).push(recipient.clone());
                    }
                }
            });
        });

        self
    }
}

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
            // let mut response = SessionManagementPacketResultBuilder::default();
            // response.session_state(SessionState::Authenticated).packet(SASLSuccess::default().into());

            // if let Ok(res) = response.build() {
            //     res.send(&msg.referer)
            // }
        }
    }
}

#[derive(Message)]
#[rtype("()")]
pub struct AuthenticationRequest {
    packet: Auth,
    // referer: Sender<SessionManagementPacketResult>,
}

impl AuthenticationRequest {
    pub(crate) fn new(packet: Auth) -> Self {
        Self { packet }
    }
}
