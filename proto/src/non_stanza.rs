mod auth;
mod open_stream;
mod proceed_tls;
mod start_tls;
mod stream_features;

pub use auth::*;
pub use open_stream::*;
pub use proceed_tls::*;
pub use start_tls::*;
pub use stream_features::*;
use xmpp_xml::Element;

use crate::{ns, ToXmlElement};

/// Define a sub part of a Packet, a NonStanza is the representation of an XML Stream event.
/// It's used by the system to deal with the communication between entities over a network.
#[derive(Debug, Clone)]
pub enum NonStanza {
    OpenStream(OpenStream),
    ProceedTls(ProceedTls),
    StartTls(StartTls),
    SASLSuccess,
    StreamFeatures(StreamFeatures),
    Auth(Auth),
}

impl ToXmlElement for NonStanza {
    type Error = std::io::Error;

    fn to_element(&self) -> Result<Element, Self::Error> {
        match self {
            NonStanza::OpenStream(s) => s.to_element(),
            NonStanza::StreamFeatures(s) => s.to_element(),
            NonStanza::StartTls(_) => Err(std::io::Error::new(std::io::ErrorKind::Other, "shouldn't be sent back")),
            NonStanza::ProceedTls(s) => s.to_element(),
            NonStanza::SASLSuccess => Ok(Element::new((ns::SASL, "success"))),
            NonStanza::Auth(_) => Err(std::io::Error::new(std::io::ErrorKind::Other, "shouldn't be sent back")),
        }
    }
}
