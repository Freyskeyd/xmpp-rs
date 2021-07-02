use crate::messages::system::{GetMechanisms, RegisterSession, RegistrationStatus};
use actix::{Actor, Context, Handler, Supervised, SystemService};
use log::trace;
use xmpp_proto::Features;

/// Manage sessions on a node
#[derive(Default)]
pub struct SessionManager {
    // sessions: HashMap<String, HashMap<String, Session>>,
}

impl SessionManager {
    pub(crate) fn new() -> Self {
        // Self { sessions: HashMap::new() }
        Self {}
    }
}

impl Supervised for SessionManager {}
impl SystemService for SessionManager {}
impl Actor for SessionManager {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        trace!("SessionManager started");
    }
}

impl Handler<GetMechanisms> for SessionManager {
    type Result = Result<Features, ()>;

    fn handle(&mut self, _: GetMechanisms, _ctx: &mut Self::Context) -> Self::Result {
        Ok(Features::Mechanisms(vec!["PLAIN".into()]))
    }
}

impl Handler<RegisterSession> for SessionManager {
    type Result = Result<(), ()>;

    fn handle(&mut self, msg: RegisterSession, _ctx: &mut Self::Context) -> Self::Result {
        println!("Registering session");

        let _ = msg.referer.do_send(RegistrationStatus {});
        Ok(())
    }
}
