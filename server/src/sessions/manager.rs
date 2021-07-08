use std::{collections::HashMap, convert::TryFrom, thread, time::Duration};

use crate::messages::system::{GetMechanisms, RegisterSession, RegistrationStatus, SessionCommand, SessionCommandAction, UnregisterSession};
use actix::{Actor, ActorFutureExt, AsyncContext, Context, Handler, Recipient, Supervised, SystemService, WrapFuture};
use jid::BareJid;
use log::trace;
use xmpp_proto::Features;

type JidString = String;
type Resource = String;

/// Manage sessions on a node
#[derive(Default)]
pub struct SessionManager {
    sessions: HashMap<JidString, HashMap<Resource, Recipient<SessionCommand>>>,
}

impl SessionManager {
    pub(crate) fn new() -> Self {
        Self { sessions: HashMap::new() }
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

    fn handle(&mut self, msg: RegisterSession, ctx: &mut Self::Context) -> Self::Result {
        println!("Registering session");

        let resource = msg.jid.resource.clone();
        let sessions = self.sessions.entry(BareJid::try_from(msg.jid).unwrap().to_string()).or_default();

        if let Some(_) = sessions.get(&resource) {
            println!("Session already exists");
            return Err(());
        }

        sessions.insert(resource, msg.referer.clone());

        println!("Sessions: {:?}", self.sessions);

        Ok(())
    }
}

impl Handler<UnregisterSession> for SessionManager {
    type Result = Result<(), ()>;

    fn handle(&mut self, msg: UnregisterSession, ctx: &mut Self::Context) -> Self::Result {
        println!("Unregistering session");

        let resource = msg.jid.resource.clone();
        let bare_jid = BareJid::try_from(msg.jid).unwrap().to_string();
        if let Some(sessions) = self.sessions.get_mut(&bare_jid) {
            sessions.remove(&resource);
            println!("Session removed: {:?}", resource);
            if sessions.is_empty() {
                self.sessions.remove(&bare_jid);
                println!("No session for {:?} removed whole map", bare_jid);
            }
        }

        println!("Sessions: {:?}", self.sessions);

        Ok(())
    }
}
