use xmpp_xml::Element;

use crate::{FromXmlElement, NonStanza, Packet, PacketParsingError};

#[derive(Debug, Clone)]
pub struct StartTls {}

impl From<StartTls> for Packet {
    fn from(s: StartTls) -> Self {
        NonStanza::StartTls(s).into()
    }
}

impl FromXmlElement for StartTls {
    type Error = PacketParsingError;

    fn from_element(_: Element) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        Ok(Self {})
    }
}
