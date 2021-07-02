use crate::messages::AuthenticationRequest;
use crate::CONFIG;
use actix::prelude::*;
use log::trace;
use std::collections::HashMap;

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
                        self.authenticators.entry(vhost.clone()).or_insert_with(Vec::new).push(recipient.clone());
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
