use config::XMPPConfig;
use std::fmt::Debug;

pub trait EventTrait: Debug + ToString + Clone {}

mod open_stream;
mod proceed_tls;
mod success_tls;
mod start_tls;
mod stream_features;
mod unknown;
mod presence;
mod auth;
mod bind;

pub use events::open_stream::OpenStream;
pub use events::stream_features::StreamFeatures;
pub use events::proceed_tls::ProceedTls;
pub use events::success_tls::SuccessTls;
pub use events::start_tls::StartTls;
pub use events::unknown::Unknown;
pub use events::presence::Presence;
pub use events::auth::Auth;
pub use events::bind::Bind;
pub use events::bind::Generic;

impl EventTrait for OpenStream {}
impl EventTrait for ProceedTls {}
impl EventTrait for SuccessTls {}
impl EventTrait for StartTls {}
impl EventTrait for StreamFeatures {}
impl EventTrait for Unknown {}
impl EventTrait for Presence {}
impl EventTrait for Auth {}
impl EventTrait for Bind {}
impl EventTrait for Generic {}

#[derive(Debug, Clone)]
pub enum IqType {
    Bind(Bind),
    Generic(Generic),
}
#[derive(Debug, Clone)]
pub enum StanzaEvent {
    Presence(Presence),
    Iq(IqType),
    IqRequest(IqType),
    IqResponse(IqType)
}

#[derive(Debug, Clone)]
pub enum NonStanzaEvent {
    OpenStream(OpenStream),
    ProceedTls(ProceedTls),
    SuccessTls(SuccessTls),
    StartTls(StartTls),
    StreamFeatures(StreamFeatures),
    Auth(Auth),
}

#[derive(Debug, Clone)]
pub enum Event {
    Unknown(Unknown, String),
    NonStanza(NonStanzaEvent, String),
    Stanza(StanzaEvent, String),
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use super::*;

    fn compile<M: EventTrait>(event: &M) -> String {
        event.to_string()
    }

    #[test]
    fn test_event() {
        let event = OpenStream::new(&XMPPConfig::new());

        let initial_stream = "<stream:stream version='1.0' xmlns:stream='http://etherx.jabber.org/streams' to='example.com' xmlns='jabber:client'>";

        assert!(compile(&event) == initial_stream.to_string(), compile(&event));
    }

    #[test]
    fn test_parse() {
        let initial_stream = "<stream:stream version='1.0' xmlns:stream='http://etherx.jabber.org/streams' to='example.com' xmlns='jabber:client'>";

        let event = OpenStream::from_str(initial_stream).unwrap();

        assert!(event.to == Some("example.com".to_string()));
        assert!(event.xmlns == "jabber:client");
    }
}

