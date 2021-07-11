use actix::Recipient;
use jid::Jid;
use tokio::sync::mpsc::Sender;

use crate::messages::{system::SessionCommand, SessionManagementPacketResult};

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum SessionState {
    Opening,
    Negociating,
    Negociated,
    Authenticating,
    Authenticated,
    Binding,
    Binded,
    Closing,

    UnsupportedEncoding,
}

impl Default for SessionState {
    fn default() -> Self {
        SessionState::Opening
    }
}

#[derive(derive_builder::Builder, Debug, Clone)]
#[builder(setter(into))]
pub(crate) struct StaticSessionState {
    #[builder(default = "SessionState::Opening")]
    pub(crate) state: SessionState,
    #[builder(default = "None")]
    pub(crate) jid: Option<Jid>,
    #[builder(default = "None")]
    pub(crate) addr_session_command: Option<Recipient<SessionCommand>>,
    #[builder(default = "ResponseAddr::Nothing")]
    pub(crate) addr_response: ResponseAddr,
}

impl StaticSessionState {
    pub(crate) fn builder() -> StaticSessionStateBuilder {
        StaticSessionStateBuilder::default()
    }

    pub(crate) fn get_addr(&self) -> Option<Recipient<SessionCommand>> {
        self.addr_session_command.clone()
    }

    pub(crate) fn get_responder(&self) -> ResponseAddr {
        self.addr_response.clone()
    }

    pub(crate) fn set_jid(mut self, jid: Jid) -> Self {
        self.jid = Some(jid);

        self
    }
}

#[derive(Debug, Clone)]
pub(crate) enum ResponseAddr {
    #[allow(dead_code)]
    Authenticated(Recipient<SessionManagementPacketResult>),
    #[allow(dead_code)]
    Unauthenticated(Sender<SessionManagementPacketResult>),
    Nothing,
}
