use actix::{Message, Recipient};
use jid::Jid;
use xmpp_proto::{Features, Packet};

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
    SendPacket(Packet),
}

/// PacketOut represents a set of packets to be sent throught the sink.
/// This kind of packet must be exchange between raw session and Session.
#[derive(Debug, Message)]
#[rtype("Result<(),()>")]
pub(crate) struct PacketsOut(pub(crate) Vec<Packet>);

/// PacketIn represents a packet received in a stream.
/// This kind of packet must be exchange between raw session and Session.
#[derive(Debug, Message)]
#[rtype("()")]
pub(crate) struct PacketIn(pub(crate) Packet);
