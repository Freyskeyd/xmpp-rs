use std::io;
use std::fmt;
use std::str::FromStr;
use std::fmt::Debug;
use xmpp_xml::Element;

pub trait FromGeneric {
    type Generic;
    type Out;
    fn from_generic(event: &Self::Generic) -> Result<Self::Out, io::Error>;
}

/// Define a sub part of an Event, an IqEvent is the representation of a Stanza which is an IQ.
///
/// IqEvent have some predefined types (BindEvent, PingEvent, ...) but the main part is
/// GenericEvent. GenericEvent will hold every incoming or outgoing events (for IQ).
/// It's a simple way to communicate.
///
/// A PingEvent, when sent to a stream, will first compile as GenericIq and add some particular
/// element or attributes to produce a ping XML Element.
#[derive(Debug, Clone)]
pub enum IqEvent {
    BindEvent(super::Bind),
    GenericEvent(super::GenericIq),
    PingEvent(super::Ping),
}

/// Define a sub part of an Event, a StanzaEvent is the representation of a Stanza which can be a
/// Presence, an IQ or a Message.
#[derive(Debug, Clone)]
pub enum StanzaEvent {
    PresenceEvent(super::Presence),
    IqEvent(Box<IqEvent>),
    IqRequestEvent(Box<IqEvent>),
    IqResponseEvent(Box<IqEvent>),
    MessageEvent(Box<super::GenericMessage>),
}

/// Define a sub part of an Event, a NonStanzaEvent is the representation of an XML Stream event.
/// It's used by the system to deal with the communication between entities over a network.
#[derive(Debug, Clone)]
pub enum NonStanzaEvent {
    OpenStreamEvent(Box<super::OpenStream>),
    CloseStreamEvent,
    ProceedTlsEvent(Box<super::ProceedTls>),
    SuccessTlsEvent(Box<super::SuccessTls>),
    StartTlsEvent(Box<super::StartTls>),
    StreamFeaturesEvent(Box<super::StreamFeatures>),
    AuthEvent(Box<super::Auth>),
}

/// Define an Event between a client and a server (or a server and another server)
///
/// An Event has many types. First, it can be an a NonStanza or a Stanza.
///
/// - A NonStanza is an XML Stream, an XML stream is a container for the exchange of XML elements between any two entities over a network.
/// - A Stanza is an XML Stanza, an XML stanza is a discrete semantic unit of structured information that is sent from one entity to another over an XML stream.
///
/// a ping Event is define like this:
///
/// `Event::Stanza(Box<StanzaEvent::IqEvent(Box<IqEvent::PingEvent(Ping)>)>)`
///
/// It complicated but every Event can be instanciate in a more clearer way:
///
/// ```rust
/// use xmpp_proto::events::{ToEvent, Event, IqEvent, Message, Ping,  StanzaEvent};
/// use xmpp_proto::Jid;
/// use std::str::FromStr;
///
/// let from = Jid::from_str("from_jid").unwrap();
/// let to = Jid::from_str("to_jid").unwrap();
///
/// // Create a Ping event (which is a struct with some Derive and implementing ToEvent)
/// let ping = Ping::new(from, to);
///
/// // ToEvent will transform a simple struct into a full path Stanza/NonStanza
/// let x = ping.to_event(); // produce the full event path
///
///
/// // We can check that the produced event are exactly what we want,
/// // A Stanza which is an IqEvent (can be both Error,Result,Get,Set)
/// // Of the predefined type: PingEvent
/// match x {
///     Event::Stanza(stanza) => match *stanza {
///         StanzaEvent::IqEvent(iq) => match *iq {
///             IqEvent::PingEvent(ref ping) => { 
///                 // Deal with ping
///             },
///             _ => {}
///         },
///         _ => {}
///     },
///     _ => {}
/// }
/// ```
#[derive(Debug, Clone)]
pub enum Event {
    /// Represent an event which is an XML Stream
    NonStanza(Box<NonStanzaEvent>),
    /// Represent an event which is an XML Stanza
    Stanza(Box<StanzaEvent>),
}

impl Event {

    /// Test if the current Event is a Message event type
    ///
    /// # Examples
    /// ```
    /// use xmpp_proto::events::{ToEvent, Message};
    /// use xmpp_proto::Jid;
    /// use std::str::FromStr;
    ///
    /// let m = Message::new(Jid::from_str("someJId").unwrap(), "heyy");
    ///
    /// let event = m.to_event();
    ///
    /// assert!(event.is_message());
    /// ```
    pub fn is_message(&self) -> bool {
        match *self {
            Event::Stanza(ref stanza) => {
                match **stanza {
                    StanzaEvent::MessageEvent(_) => true,
                    _ => false,
                }
            }
            _ => false,
        }
    }

    /// Test if the current Event is a Presence event type
    ///
    /// # Examples
    /// ```
    /// use xmpp_proto::events::{ToEvent, Message};
    /// use xmpp_proto::Jid;
    /// use std::str::FromStr;
    ///
    /// let m = Message::new(Jid::from_str("someJId").unwrap(), "heyy");
    ///
    /// let event = m.to_event();
    ///
    /// assert!(!event.is_presence());
    /// ```
    pub fn is_presence(&self) -> bool {
        match *self {
            Event::Stanza(ref stanza) => {
                match **stanza {
                    StanzaEvent::PresenceEvent(_) => true,
                    _ => false,
                }
            }
            _ => false,
        }
    }

    /// Test if the current Event is an IQ event type
    ///
    /// # Examples
    /// ```
    /// use xmpp_proto::events::{ToEvent, Message};
    /// use xmpp_proto::Jid;
    /// use std::str::FromStr;
    ///
    /// let m = Message::new(Jid::from_str("someJId").unwrap(), "heyy");
    ///
    /// let event = m.to_event();
    ///
    /// assert!(!event.is_iq());
    /// ```
    pub fn is_iq(&self) -> bool {
        match *self {
            Event::Stanza(ref stanza) => {
                match **stanza {
                    StanzaEvent::IqEvent(_) |
                    StanzaEvent::IqRequestEvent(_) |
                    StanzaEvent::IqResponseEvent(_) => true,
                    _ => false,
                }
            }
            _ => false,
        }
    }

    /// Test if the current Event is a NonStanza event type
    ///
    /// # Examples
    /// ```
    /// use xmpp_proto::events::{ToEvent, Message};
    /// use xmpp_proto::Jid;
    /// use std::str::FromStr;
    ///
    /// let m = Message::new(Jid::from_str("someJId").unwrap(), "heyy");
    ///
    /// let event = m.to_event();
    ///
    /// assert!(!event.is_non_stanza());
    /// ```
    pub fn is_non_stanza(&self) -> bool {
        match *self {
            Event::NonStanza(_) => true,
            _ => false,
        }
    }
}

impl ToXmlElement for StanzaEvent {
    type Error = io::Error;
    fn to_element(&self) -> Result<Element, Self::Error> {
        match *self {
            StanzaEvent::PresenceEvent(ref event) => event.to_element(),
            StanzaEvent::MessageEvent(ref event) => event.to_element(),
            StanzaEvent::IqResponseEvent(ref boxed_iq) |
            StanzaEvent::IqEvent(ref boxed_iq) |
            StanzaEvent::IqRequestEvent(ref boxed_iq) => {
                match **boxed_iq {
                    IqEvent::PingEvent(ref event) => event.to_element(),
                    IqEvent::BindEvent(ref event) => event.to_element(),
                    IqEvent::GenericEvent(ref event) => event.to_element(),
                }
            }
        }
    }
}

/// ToXmlElement is used to transform any struct to an XML Element
pub trait ToXmlElement {
    type Error;
    fn to_element(&self) -> Result<Element, Self::Error>;
}

/// FromXmlElement is used to transform an Element to an object
pub trait FromXmlElement {
    type Error;
    fn from_element(e: Element) -> Result<Self, Self::Error> where Self: Sized;
}

/// ToEvent is used to transform an object to an XMPP Event
pub trait ToEvent: Debug + Clone {
    fn to_event(&self) -> Event;
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EventType {
    Iq,
    Message,
    Presence,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum IqType {
    Get,
    Set,
    Result,
    Error,
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

impl fmt::Display for IqType {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", match *self {
            IqType::Get => "get".to_string(),
            IqType::Set => "set".to_string(),
            IqType::Result => "result".to_string(),
            IqType::Error => "error".to_string(),
        })
    }
}
