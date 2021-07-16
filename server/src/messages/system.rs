use actix::{Message, Recipient};
use jid::Jid;
use xmpp_proto::{Features, Packet};

#[derive(Debug, Message)]
#[rtype("Result<(),()>")]
pub(crate) struct RegistrationStatus {}

#[derive(Debug, Message)]
#[rtype("Result<(),()>")]
pub(crate) struct RegisterSession {
    pub(crate) jid: Jid,
    pub(crate) referer: Recipient<SessionCommand>,
}

#[derive(Debug, Message)]
#[rtype("Result<(),()>")]
pub(crate) struct UnregisterSession {
    pub(crate) jid: Jid,
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

#[derive(Debug, Message)]
#[rtype("Result<(),()>")]
pub(crate) struct PacketsOut(pub(crate) Vec<Packet>);

#[derive(Debug, Message)]
#[rtype("()")]
pub(crate) struct PacketIn(pub(crate) Packet);
