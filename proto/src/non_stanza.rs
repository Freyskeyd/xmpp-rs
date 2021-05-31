use xmpp_xml::Element;

mod auth;
mod bind;
mod close_stream;
mod open_stream;
mod proceed_tls;
mod sasl_success;
mod start_tls;
mod stream_error;
mod stream_features;

pub use auth::*;
pub use bind::*;
pub use close_stream::*;
pub use open_stream::*;
pub use proceed_tls::*;
pub use sasl_success::*;
pub use start_tls::*;
pub use stream_error::*;
pub use stream_features::*;

use crate::ToXmlElement;

/// Define a sub part of a Packet, a NonStanza is the representation of an XML Stream event.
/// It's used by the system to deal with the communication between entities over a network.
#[derive(Debug, Clone)]
pub enum NonStanza {
    // TODO: Rename to Initial stream header?
    OpenStream(OpenStream),
    ProceedTls(ProceedTls),
    StartTls(StartTls),
    SASLSuccess(SASLSuccess),
    StreamFeatures(StreamFeatures),
    Auth(Auth),
    Bind(Bind),
    StreamError(StreamError),
    CloseStream(CloseStream),
}

impl ToXmlElement for NonStanza {
    type Error = std::io::Error;

    fn to_element(&self) -> Result<Element, Self::Error> {
        match self {
            NonStanza::Auth(_) => Err(std::io::Error::new(std::io::ErrorKind::Other, "shouldn't be sent back")),
            NonStanza::Bind(s) => s.to_element(),
            NonStanza::OpenStream(s) => s.to_element(),
            NonStanza::ProceedTls(s) => s.to_element(),
            NonStanza::SASLSuccess(s) => s.to_element(),
            NonStanza::StartTls(_) => Err(std::io::Error::new(std::io::ErrorKind::Other, "shouldn't be sent back")),
            NonStanza::StreamFeatures(s) => s.to_element(),
            NonStanza::StreamError(s) => s.to_element(),
            NonStanza::CloseStream(s) => s.to_element(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_element() {
        assert!(matches!(NonStanza::OpenStream(OpenStream::default()).to_element(), Ok(_)));
        assert!(matches!(NonStanza::StreamFeatures(StreamFeatures::default()).to_element(), Ok(_)));
        assert!(matches!(NonStanza::StartTls(StartTls::default()).to_element(), Err(_)));
        assert!(matches!(NonStanza::ProceedTls(ProceedTls::default()).to_element(), Ok(_)));
        assert!(matches!(NonStanza::SASLSuccess(SASLSuccess::default()).to_element(), Ok(_)));
        assert!(matches!(NonStanza::Auth(Auth::default()).to_element(), Err(_)));
    }
}
