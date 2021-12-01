use crate::messages::system::SessionCommand;
use actix::Recipient;
use jid::Jid;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum SessionState {
    Opening,
    Negociating,
    Negociated,
    Authenticating,
    Authenticated,
    Binding,
    Bound,
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
}

impl Default for StaticSessionState {
    fn default() -> Self {
        StaticSessionStateBuilder::default().build().unwrap()
    }
}
impl StaticSessionState {
    pub(crate) fn builder() -> StaticSessionStateBuilder {
        StaticSessionStateBuilder::default()
    }

    pub(crate) fn get_addr(&self) -> Option<Recipient<SessionCommand>> {
        self.addr_session_command.clone()
    }

    pub(crate) fn set_jid(mut self, jid: Jid) -> Self {
        self.jid = Some(jid);

        self
    }

    /// Set the static session state's state.
    pub(crate) fn set_state(mut self, state: SessionState) -> Self {
        self.state = state;

        self
    }
}

impl From<SessionState> for StaticSessionState {
    fn from(state: SessionState) -> Self {
        Self {
            state,
            jid: None,
            addr_session_command: None,
        }
    }
}
