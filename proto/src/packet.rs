use crate::{non_stanza::StartTls, ns, stanza::GenericIq, Auth, OpenStream, StreamFeatures, ToXmlElement};
use crate::{FromXmlElement, ProceedTls};
use crate::{NonStanza, NonStanzaTrait, Stanza};

use actix::Message;
use circular::Buffer;
use std::{
    convert::{TryFrom, TryInto},
    io::Write,
};
use xmpp_xml::WriteOptions;
use xmpp_xml::{
    xml::{attribute::OwnedAttribute, name::OwnedName, namespace::Namespace, EventReader},
    Element,
};

#[derive(Debug, Message, Clone)]
#[rtype(result = "Result<Vec<Packet>, ()>")]
pub enum Packet {
    /// Represent a packet which is an XML Stream
    NonStanza(Box<NonStanza>),
    /// Represent a packet which isn't an XML Stanza
    Stanza(Box<Stanza>),
}

impl<T> From<T> for Packet
where
    T: NonStanzaTrait,
{
    fn from(s: T) -> Self {
        s.into()
    }
}

impl From<NonStanza> for Packet {
    fn from(s: NonStanza) -> Self {
        Packet::NonStanza(Box::new(s))
    }
}

impl From<Stanza> for Packet {
    fn from(s: Stanza) -> Self {
        Packet::Stanza(Box::new(s))
    }
}

#[derive(Debug)]
pub enum PacketParsingError {
    Final,
    Xml(xmpp_xml::Error),
    Io,
    Unknown,
}

impl From<xmpp_xml::Error> for PacketParsingError {
    fn from(e: xmpp_xml::Error) -> Self {
        PacketParsingError::Xml(e)
    }
}

impl From<std::io::Error> for PacketParsingError {
    fn from(_: std::io::Error) -> Self {
        PacketParsingError::Io
    }
}

impl TryFrom<Element> for Packet {
    type Error = PacketParsingError;

    fn try_from(element: Element) -> Result<Self, Self::Error> {
        match (element.tag().ns(), element.tag().name()) {
            (Some(ns::STREAM), "features") => Ok(StreamFeatures::from_element(&element)?.into()),
            (Some(ns::SASL), "auth") => Ok(Auth::from_element(&element)?.into()),
            (Some(ns::TLS), "starttls") => Ok(StartTls::from_element(&element)?.into()),
            (Some(ns::TLS), "proceed") => Ok(ProceedTls::from_element(&element)?.into()),
            (Some(ns::CLIENT), "iq") => Ok(GenericIq::from_element(&element)?.into()),
            (None, "message") => Ok(Stanza::Message(element).into()),
            (None, "presence") => Ok(Stanza::Presence(element).into()),
            e => {
                println!("{:?}", e);
                Err(PacketParsingError::Unknown)
            }
        }
    }
}

impl Packet {
    pub fn write_to_stream<W: Write>(&self, stream: W) -> Result<(), std::io::Error> {
        match self {
            Packet::NonStanza(s) => Ok(s.to_element()?.to_writer_with_options(stream, WriteOptions::new().set_xml_prolog(None))?),
            Packet::Stanza(s) => Ok(s.to_element()?.to_writer_with_options(stream, WriteOptions::new().set_xml_prolog(None))?),
        }
    }

    pub fn parse(buffer: &mut EventReader<Buffer>, name: OwnedName, namespace: Namespace, attributes: Vec<OwnedAttribute>) -> Result<Self, PacketParsingError> {
        match (name.namespace_ref(), name.local_name.as_ref()) {
            // open stream isn't an element
            (Some(ns::STREAM), "stream") => Ok(OpenStream::from_start_element(attributes)?.into()),
            _ => Element::from_start_element(name, attributes, namespace, None, buffer)?.try_into(),
        }
    }
}
