use crate::messages::AuthenticationRequest;
use crate::messages::SessionManagementPacketResultBuilder;
use crate::sessions::state::{SessionState, StaticSessionState};
use crate::CONFIG;
use actix::prelude::*;
use base64::decode;
use jid::Jid;
use log::trace;
use sasl::common::Identity;
use sasl::common::Identity::Username;
use sasl::secret;
use sasl::server::mechanisms::Plain as ServerPlain;
use sasl::server::Mechanism;
use sasl::server::Response;
use sasl::server::Validator;
use sasl::server::ValidatorError;
use std::collections::HashMap;
use std::str::FromStr;
use xmpp_proto::SASLSuccess;

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
            println!("AUTHENT WITH PLAIN");
            let challenge = decode(msg.packet.challenge().as_ref().unwrap()).unwrap();

            let mut mech = ServerPlain::new(MyValidator);
            println!("{:?}", mech.respond(&challenge));
            let username = match mech.respond(&challenge) {
                Ok(Response::Success(Username(username), _)) => username,
                _ => {
                    return ();
                }
            };
            let mut response = SessionManagementPacketResultBuilder::default();
            response
                .real_session_state(
                    StaticSessionState::builder()
                        .jid(Some(Jid::from_str(&format!("{}@localhost", username)).unwrap()))
                        .state(SessionState::Authenticated)
                        .build()
                        .unwrap(),
                )
                .session_state(SessionState::Authenticated)
                .packet(SASLSuccess::default().into());

            if let Ok(res) = response.build() {
                match msg.from {
                    crate::sessions::state::ResponseAddr::Unauthenticated(from) => res.send(Some(from)),
                    _ => panic!("Shouldnt happend"),
                }
            }
        }
    }
}
const USERNAME: &'static str = "local";
const PASSWORD: &'static str = "admin";
struct MyValidator;
impl Validator<secret::Plain> for MyValidator {
    fn validate(&self, identity: &Identity, value: &secret::Plain) -> Result<(), ValidatorError> {
        let &secret::Plain(ref password) = value;
        if identity != &Identity::Username(USERNAME.to_owned()) {
            Err(ValidatorError::AuthenticationFailed)
        } else if password != PASSWORD {
            Err(ValidatorError::AuthenticationFailed)
        } else {
            Ok(())
        }
    }
}
