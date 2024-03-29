use crate::messages::system::{GetMechanisms, RegisterSession, SessionCommand, UnregisterSession};
use actix::{Actor, Context, Handler, Recipient, Supervised, SystemService};
use jid::BareJid;
use log::trace;
use std::{collections::HashMap, convert::TryFrom};
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

    fn handle(&mut self, msg: RegisterSession, _ctx: &mut Self::Context) -> Self::Result {
        trace!("Registering session");

        if let jid::Jid::Full(jid) = msg.jid {
            let resource = jid.resource.clone();
            let sessions = self.sessions.entry(BareJid::try_from(jid).unwrap().to_string()).or_default();

            if sessions.get(&resource).is_some() {
                trace!("Session already exists");
                return Err(());
            }

            sessions.insert(resource, msg.referer.clone());

            Ok(())
        } else {
            Err(())
        }
    }
}

impl Handler<UnregisterSession> for SessionManager {
    type Result = Result<(), ()>;

    fn handle(&mut self, msg: UnregisterSession, _ctx: &mut Self::Context) -> Self::Result {
        trace!("Unregistering session");

        if let jid::Jid::Full(jid) = msg.jid {
            let resource = jid.resource.clone();
            let bare_jid = BareJid::try_from(jid).unwrap().to_string();
            if let Some(sessions) = self.sessions.get_mut(&bare_jid) {
                sessions.remove(&resource);
                trace!("Session removed: {:?}", resource);
                if sessions.is_empty() {
                    self.sessions.remove(&bare_jid);
                    trace!("No session for {:?} removed whole map", bare_jid);
                }
            }
        }
        Ok(())
    }
}
