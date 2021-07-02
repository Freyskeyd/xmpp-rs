use actix::{Message, Recipient};
use xmpp_proto::Features;

#[derive(Debug, Message)]
#[rtype("Result<(),()>")]
pub(crate) struct RegistrationStatus {}

#[derive(Debug, Message)]
#[rtype("Result<(),()>")]
pub(crate) struct RegisterSession {
    pub(crate) referer: Recipient<RegistrationStatus>,
}

#[derive(Debug, Message)]
#[rtype("Result<Features,()>")]
pub(crate) struct GetMechanisms(pub(crate) String);
