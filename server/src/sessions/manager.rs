use std::{collections::HashMap, convert::TryFrom};

use crate::messages::system::{GetMechanisms, RegisterSession, SessionCommand, UnregisterSession};
use actix::{Actor, Context, Handler, Recipient, Supervised, SystemService};
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

    fn handle(&mut self, msg: RegisterSession, _ctx: &mut Self::Context) -> Self::Result {
        println!("Registering session");

        match msg.jid {
            jid::Jid::Full(jid) => {
                let resource = jid.resource.clone();
                let sessions = self.sessions.entry(BareJid::try_from(jid).unwrap().to_string()).or_default();

                if let Some(_) = sessions.get(&resource) {
                    println!("Session already exists");
                    return Err(());
                }

                sessions.insert(resource, msg.referer.clone());

                println!("Sessions: {:?}", self.sessions);

                Ok(())
            }
            _ => Err(()),
        }
    }
}

impl Handler<UnregisterSession> for SessionManager {
    type Result = Result<(), ()>;

    fn handle(&mut self, msg: UnregisterSession, _ctx: &mut Self::Context) -> Self::Result {
        println!("Unregistering session");

        println!("{:?}", msg);
        if let jid::Jid::Full(jid) = msg.jid {
            let resource = jid.resource.clone();
            let bare_jid = BareJid::try_from(jid.clone()).unwrap().to_string();
            if let Some(sessions) = self.sessions.get_mut(&bare_jid) {
                sessions.remove(&resource);
                println!("Session removed: {:?}", resource);
                if sessions.is_empty() {
                    self.sessions.remove(&bare_jid);
                    println!("No session for {:?} removed whole map", bare_jid);
                }
            }

            println!("Sessions: {:?}", self.sessions);
        }
        Ok(())
    }
}
