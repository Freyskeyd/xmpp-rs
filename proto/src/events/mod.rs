mod auth;
mod bind;
mod generic;
mod message;
mod open_stream;
mod close_stream;
mod ping;
mod presence;
mod proceed_tls;
mod start_tls;
mod stream_features;
mod success_tls;
mod interface;

pub use events::auth::Auth;
pub use events::bind::Bind;
pub use events::generic::GenericIq;
pub use events::generic::GenericMessage;
pub use events::generic::PresenceType;
pub use events::message::Message;
pub use events::open_stream::OpenStream;
pub use events::close_stream::CloseStream;
pub use events::ping::Ping;
pub use events::presence::Presence;
pub use events::proceed_tls::ProceedTls;
pub use events::start_tls::StartTls;
pub use events::stream_features::{StreamFeatures, Features};
pub use events::success_tls::SuccessTls;
pub use events::interface::*;

#[cfg(test)]
mod tests {
    use config::XMPPConfig;
    use events::ToEvent;
    use super::*;

    fn compile<M: ToEvent>(_: &M) -> String {
        String::new()
    }

    #[test]
    #[ignore]
    fn test_event() {
        let event = OpenStream::new(&XMPPConfig::new());

        let initial_stream = "<?xml version='1.0'?><stream:stream version='1.0' xmlns:stream='http://etherx.jabber.org/streams' to='example.com' xmlns='jabber:client'>";

        assert!(compile(&event) == initial_stream.to_string(),
                format!("{} == {}", compile(&event), initial_stream.to_string()));
    }

    // #[test]
    // fn test_parse() {
    //     let initial_stream = "<stream:stream version='1.0' xmlns:stream='http://etherx.jabber.org/streams' to='example.com' xmlns='jabber:client'>";

    //     let event = OpenStream::from_str(initial_stream).unwrap();

    //     assert!(event.to == Some("example.com".to_string()));
    //     assert!(event.xmlns == "jabber:client");
    // }
}
