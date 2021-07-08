use actix::{Message, Recipient};
use jid::FullJid;
use xmpp_proto::Features;

#[derive(Debug, Message)]
#[rtype("Result<(),()>")]
pub(crate) struct RegistrationStatus {}

#[derive(Debug, Message)]
#[rtype("Result<(),()>")]
pub(crate) struct RegisterSession {
    pub(crate) jid: FullJid,
    pub(crate) referer: Recipient<SessionCommand>,
}

#[derive(Debug, Message)]
#[rtype("Result<(),()>")]
pub(crate) struct UnregisterSession {
    pub(crate) jid: FullJid,
}

#[derive(Debug, Message)]
#[rtype("Result<Features,()>")]
pub(crate) struct GetMechanisms(pub(crate) String);

#[derive(Debug, Message)]
#[rtype("Result<(),()>")]
pub(crate) struct SessionCommand(pub(crate) SessionCommandAction);

#[derive(Debug)]
pub(crate) enum SessionCommandAction {
    Kill,
}
