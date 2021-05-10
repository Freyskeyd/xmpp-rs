use crate::{non_stanza::StartTls, ns, stanza::GenericIq, Auth, ToXmlElement};
use crate::{FromXmlElement, OpenStreamBuilder};
use crate::{NonStanza, NonStanzaTrait, Stanza};

use actix::Message;
use circular::Buffer;
use std::io::Write;
use uuid::Uuid;
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

impl Packet {
    pub fn write_to_stream<W: Write>(&self, stream: W) -> Result<(), std::io::Error> {
        match self {
            Packet::NonStanza(s) => Ok(s.to_element()?.to_writer_with_options(stream, WriteOptions::new().set_xml_prolog(None))?),
            Packet::Stanza(s) => Ok(s.to_element()?.to_writer_with_options(stream, WriteOptions::new().set_xml_prolog(None))?),
        }
    }

    pub fn parse(buffer: &mut EventReader<Buffer>, name: OwnedName, namespace: Namespace, attributes: Vec<OwnedAttribute>) -> Option<Self> {
        match name.local_name.as_ref() {
            "stream" if name.namespace_ref() == Some(ns::STREAM) => {
                let (to, lang, version) = attributes.iter().fold((String::from(""), String::from("en"), String::from("0.0")), |(to, lang, version), attribute| {
                    match attribute.name.local_name.as_ref() {
                        "to" if attribute.name.namespace.is_none() => (attribute.value.to_string(), lang, version),
                        "lang" if attribute.name.namespace == Some(ns::XML_URI.to_string()) => (to, attribute.value.to_string(), version),
                        "version" if attribute.name.namespace.is_none() => (to, lang, attribute.value.to_string()),
                        _ => (to, lang, version),
                    }
                });
                let e = OpenStreamBuilder::default().id(Uuid::new_v4()).to(to).lang(lang).version(version).build().unwrap();

                Some(e.into())
            }
            "auth" if name.namespace_ref() == Some(ns::SASL) => Element::from_start_element(name, attributes, namespace, None, buffer).map_or(None, |e| Some(Auth::from_element(e).unwrap().into())),
            "starttls" if name.namespace_ref() == Some(ns::TLS) => Element::from_start_element(name, attributes, namespace, None, buffer).map_or(None, |_| Some(StartTls {}.into())),
            "iq" => Element::from_start_element(name, attributes, namespace, None, buffer).map_or(None, |e| Some(GenericIq::from_element(e).unwrap().into())),
            "message" => Element::from_start_element(name, attributes, namespace, None, buffer).map_or(None, |e| Some(Stanza::Message(e).into())),
            "presence" => Element::from_start_element(name, attributes, namespace, None, buffer).map_or(None, |e| Some(Stanza::Presence(e).into())),
            _ => None,
        }
    }
}
