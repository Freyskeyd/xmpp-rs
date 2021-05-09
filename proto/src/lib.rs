mod non_stanza;
use circular::Buffer;
use jid::Jid;
use std::io::Write;
use uuid::Uuid;

use actix::Message;
pub use non_stanza::*;
use xmpp_xml::{
    xml::{attribute::OwnedAttribute, name::OwnedName, namespace::Namespace, EventReader},
    Element, WriteOptions,
};

#[derive(Debug, Message, Clone)]
#[rtype(result = "Result<Vec<Packet>, ()>")]
pub enum Packet {
    /// Represent a packet which is an XML Stream
    NonStanza(Box<NonStanza>),
    /// Represent a packet which isn't an XML Stanza
    Stanza(Box<Stanza>),
}

pub trait NonStanzaTrait {}

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

/// Define a sub part of a Packet, a Stanza is the representation of an Xmpp Stanza which can be a
/// Presence, an IQ or a Message.
#[derive(Debug, Clone)]
pub enum Stanza {
    IQ(GenericIq),
    Message(Element),
    Presence(Element),
}

impl ToXmlElement for Element {
    type Error = io::Error;

    fn to_element(&self) -> Result<Element, Self::Error> {
        Ok(self.clone())
    }
}

impl ToXmlElement for Stanza {
    type Error = std::io::Error;

    fn to_element(&self) -> Result<Element, Self::Error> {
        match self {
            Stanza::IQ(iq) => iq.to_element(),
            Stanza::Message(s) => s.to_element(),
            Stanza::Presence(s) => s.to_element(),
        }
    }
}

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

/// FromXmlElement is used to transform an Element to an object
pub trait FromXmlElement {
    type Error;
    fn from_element(e: Element) -> Result<Self, Self::Error>
    where
        Self: Sized;
}

pub trait ToXmlElement {
    type Error;

    fn to_element(&self) -> Result<Element, Self::Error>;
}

pub mod ns {
    pub const XML_URI: &'static str = "http://www.w3.org/XML/1998/namespace";
    pub const CLIENT: &'static str = "jabber:client";
    pub const SERVER: &'static str = "jabber:server";
    pub const STREAM: &'static str = "http://etherx.jabber.org/streams";
    pub const TLS: &'static str = "urn:ietf:params:xml:ns:xmpp-tls";
    pub const SASL: &'static str = "urn:ietf:params:xml:ns:xmpp-sasl";
    pub const BIND: &'static str = "urn:ietf:params:xml:ns:xmpp-bind";
    pub const SESSION: &'static str = "urn:ietf:params:xml:ns:xmpp-session";
    pub const STANZAS: &'static str = "urn:ietf:params:xml:ns:xmpp-stanzas";
    pub const PING: &'static str = "urn:xmpp:ping";
}

#[derive(Debug, Clone, PartialEq)]
pub enum Features {
    StartTlsInit,
    Bind,
    Mechanisms(Vec<String>),
    Unknown,
}

#[derive(Debug, Clone)]
pub struct StreamFeatures {
    pub features: Features,
}

impl From<StreamFeatures> for Packet {
    fn from(s: StreamFeatures) -> Self {
        NonStanza::StreamFeatures(s).into()
    }
}

impl ToXmlElement for StreamFeatures {
    type Error = std::io::Error;
    fn to_element(&self) -> Result<Element, std::io::Error> {
        let mut root = Element::new("stream:features");

        match self.features {
            Features::StartTlsInit => {
                let starttls = root.append_new_child((ns::TLS, "starttls"));
                starttls.append_new_child((ns::TLS, "required"));
            }
            Features::Bind => {
                root.append_new_child((ns::BIND, "bind"));
            }
            Features::Mechanisms(ref mechanisms) => {
                let node = root.append_new_child((ns::SASL, "mechanisms"));
                mechanisms.iter().for_each(|mech| {
                    node.append_new_child((ns::SASL, "mechanism")).set_text(mech);
                });
            }
            Features::Unknown => {}
        }

        Ok(root)
    }
}

#[derive(Debug, Clone)]
pub struct StartTls {}

impl From<StartTls> for Packet {
    fn from(s: StartTls) -> Self {
        NonStanza::StartTls(s).into()
    }
}

#[derive(Debug, Clone)]
pub struct GenericIq {
    id: String,
    iq_type: IqType,
    to: Option<Jid>,
    from: Option<Jid>,
    element: Option<Element>,
    // error: Option<StanzaError>,
}

impl From<GenericIq> for Packet {
    fn from(s: GenericIq) -> Self {
        Stanza::IQ(s).into()
    }
}
impl Default for GenericIq {
    fn default() -> Self {
        Self::new("", IqType::Get)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum IqType {
    Get,
    Set,
    Result,
    Error,
}

impl Into<String> for IqType {
    fn into(self) -> String {
        match self {
            IqType::Get => "get".to_string(),
            IqType::Set => "set".to_string(),
            IqType::Result => "result".to_string(),
            IqType::Error => "error".to_string(),
        }
    }
}

impl GenericIq {
    pub fn new<T: ToString + ?Sized>(id: &T, iq_type: IqType) -> GenericIq {
        GenericIq {
            id: id.to_string(),
            iq_type: iq_type,
            to: None,
            from: None,
            element: None,
        }
    }

    pub fn unique_id() -> String {
        Uuid::new_v4().to_string()
    }

    pub fn get_element(&self) -> Option<&Element> {
        self.element.as_ref()
    }

    pub fn set_type(&mut self, iq_type: IqType) -> &mut Self {
        self.iq_type = iq_type;
        self
    }

    pub fn get_type(&self) -> IqType {
        self.iq_type
    }

    pub fn get_id(&self) -> &str {
        self.id.as_ref()
    }

    pub fn set_id<T: ToString + ?Sized>(&mut self, id: &T) -> &mut Self {
        self.id = id.to_string();
        self
    }

    pub fn set_to(&mut self, jid: Option<Jid>) -> &mut Self {
        self.to = jid;

        self
    }

    pub fn get_to(&self) -> Option<&Jid> {
        self.to.as_ref()
    }

    pub fn set_from(&mut self, jid: Option<Jid>) -> &mut Self {
        self.from = jid;

        self
    }

    pub fn get_from(&self) -> Option<&Jid> {
        self.from.as_ref()
    }
}
use std::fmt;
use std::io;
use std::str::FromStr;

impl ToXmlElement for GenericIq {
    type Error = io::Error;

    fn to_element(&self) -> Result<Element, Self::Error> {
        println!("Encoding Generic IQ");
        Ok(self.element.clone().unwrap())
    }
}

impl FromXmlElement for GenericIq {
    type Error = io::Error;
    fn from_element(e: Element) -> Result<Self, io::Error> {
        let id = match e.get_attr("id") {
            Some(id) => id.to_string(),
            None => return Err(io::Error::new(io::ErrorKind::InvalidInput, "ID is required")),
        };

        let iq_type = match e.get_attr("type") {
            Some(t) => match IqType::from_str(t) {
                Ok(t) => t,
                Err(e) => return Err(e),
            },
            None => return Err(io::Error::new(io::ErrorKind::InvalidInput, "TYPE is required")),
        };

        // Validation types
        match iq_type {
            IqType::Result => {
                if e.child_count() > 1 {
                    return Err(io::Error::new(io::ErrorKind::InvalidInput, "An IQ stanza of type \"result\" MUST include zero or one child elements."));
                }
            }

            // An error stanza MUST contain an <error/> child element
            IqType::Error => {
                if e.find("error").is_none() {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "An IQ stanza of type \"error\" SHOULD include the child element contained in the associated \"get\" or \"set\" and MUST include an <error/> child",
                    ));
                }
            }
            IqType::Set | IqType::Get => {
                if e.child_count() != 1 {
                    // https://xmpp.org/rfcs/rfc3920.html#stanzas
                    return Err(io::Error::new(io::ErrorKind::InvalidInput, "IqType Get/Set MUST contain one and only one child"));
                }
            }
        }

        // An <error/> child MUST NOT be included if the 'type' attribute has a value other than "error"
        match iq_type {
            IqType::Set | IqType::Get | IqType::Result => {
                if e.find("error").is_some() {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "An <error/> child MUST NOT be included if the 'type' attribute has a value other than \"error\"",
                    ));
                }
            }
            _ => {}
        }

        let to = {
            if let Some(t) = e.get_attr("to") {
                match Jid::from_str(t) {
                    Ok(j) => Some(j),
                    Err(_) => None,
                }
            } else {
                None
            }
        };

        let from = {
            if let Some(f) = e.get_attr("from") {
                match Jid::from_str(f) {
                    Ok(j) => Some(j),
                    Err(_) => None,
                }
            } else {
                None
            }
        };

        // let error = {
        //     if iq_type == IqType::Error {
        //         match StanzaError::from_element(e.find("error").unwrap().to_owned()) {
        //             Ok(error_element) => Some(error_element),
        //             Err(_) => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Unparsable error element")),
        //         }
        //     } else {
        //         None
        //     }
        // };

        Ok(GenericIq {
            id,
            iq_type,
            to,
            from,
            element: Some(e.clone()),
            // error,
        })
    }
}

impl FromStr for IqType {
    type Err = io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "get" => Ok(IqType::Get),
            "set" => Ok(IqType::Set),
            "result" => Ok(IqType::Result),
            "error" => Ok(IqType::Error),
            _ => Err(io::Error::new(io::ErrorKind::InvalidInput, "Unsupported IqType")),
        }
    }
}
impl fmt::Display for IqType {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(
            f,
            "{}",
            match *self {
                IqType::Get => "get".to_string(),
                IqType::Set => "set".to_string(),
                IqType::Result => "result".to_string(),
                IqType::Error => "error".to_string(),
            }
        )
    }
}
