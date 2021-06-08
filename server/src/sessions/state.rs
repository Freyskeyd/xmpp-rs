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
