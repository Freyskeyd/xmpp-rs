#![allow(dead_code)]
use std::fmt;
use uuid::Uuid;
use std::io;
use elementtree::Element;
use std::string::ToString;
use std::str::FromStr;
use jid::{Jid, ToJid};
use events::IqType;
use events::Event;
use events::StanzaEvent;
use events::IqEvent;
use events::ToXmlElement;
use events::FromXmlElement;
use ns;

#[derive(Debug, Clone)]
pub struct GenericIq {
    id: String,
    iq_type: IqType,
    to: Option<Jid>,
    from: Option<Jid>,
    element: Option<Element>,
    error: Option<StanzaError>,
}

impl Default for GenericIq {
    fn default() -> Self {
        Self::new("", IqType::Get)
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
            error: None,
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

    pub fn to_event(&self) -> Event {
        Event::Stanza(Box::new(StanzaEvent::IqEvent(Box::new(IqEvent::GenericEvent(self.clone())))))
    }
}

impl ToXmlElement for GenericIq {
    type Error = io::Error;
    fn to_element(&self) -> Result<Element, Self::Error> {
        let mut root = Element::new("iq");
        root.set_attr("type", self.iq_type.to_string());
        root.set_attr("id", self.id.to_string());

        if let Some(to) = self.get_to() {
            root.set_attr("to", to.to_string());
        }

        if let Some(from) = self.get_from() {
            root.set_attr("from", from.to_string());
        }

        Ok(root)
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
                    return Err(io::Error::new(io::ErrorKind::InvalidInput,
                                              "An IQ stanza of type \"result\" MUST include zero or one child elements."));
                }
            }

            // An error stanza MUST contain an <error/> child element
            IqType::Error => {
                if e.find("error").is_none() {
                    return Err(io::Error::new(io::ErrorKind::InvalidInput,
                                              "An IQ stanza of type \"error\" SHOULD include the child element contained in the associated \"get\" or \"set\" and MUST include an <error/> child"));
                }
            }
            IqType::Set | IqType::Get => {
                if e.child_count() != 1 {
                    // https://xmpp.org/rfcs/rfc3920.html#stanzas
                    return Err(io::Error::new(io::ErrorKind::InvalidInput,
                                              "IqType Get/Set MUST contain one and only one child"));
                }
            }
        }

        // An <error/> child MUST NOT be included if the 'type' attribute has a value other than "error"
        match iq_type {
            IqType::Set | IqType::Get |IqType::Result => {
                if e.find("error").is_some() {
                    return Err(io::Error::new(io::ErrorKind::InvalidInput, "An <error/> child MUST NOT be included if the 'type' attribute has a value other than \"error\""));
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

        let error = {
            if iq_type == IqType::Error {
                match StanzaError::from_element(e.find("error").unwrap().to_owned()) {
                    Ok(error_element) => {
                        Some(error_element)
                    },
                    Err(_) => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Unparsable error element")),
                }

            } else {
                None
            }
        };

        Ok(GenericIq {
               id,
               iq_type,
               to,
               from,
               element: Some(e.clone()),
               error,
           })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MessageType {
    Chat,
    Error,
}

impl FromStr for MessageType {
    type Err = io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "error" => Ok(MessageType::Error),
            "chat" => Ok(MessageType::Chat),
            _ => Err(io::Error::new(io::ErrorKind::InvalidInput, "Unsupported MessageType")),
        }
    }
}

impl Into<String> for MessageType {
    fn into(self) -> String {
        match self {
            MessageType::Error => "error".to_string(),
            MessageType::Chat => "chat".to_string(),
        }
    }
}

impl fmt::Display for MessageType {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", match *self {
            MessageType::Error => "error".to_string(),
            MessageType::Chat => "chat".to_string(),
        })
    }
}


#[derive(Debug, Clone)]
pub struct GenericMessage {
    id: Option<String>,
    to: Jid,
    from: Option<Jid>,
    message_type: Option<MessageType>,
    element: Option<Element>,
    pub childs: Option<Vec<Element>>,
}

impl GenericMessage {
    pub fn new(to: Jid) -> GenericMessage {
        GenericMessage {
            id: None,
            to: to.to_jid().unwrap(),
            from: None,
            message_type: Some(MessageType::Chat),
            element: None,
            childs: None,
        }
    }

    pub fn set_type(&mut self, message_type: Option<MessageType>) -> &mut Self {
        self.message_type = message_type;
        self
    }

    pub fn get_type(&self) -> Option<&MessageType> {
        self.message_type.as_ref()
    }

    pub fn get_id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    pub fn set_id<T: ToString>(&mut self, id: Option<T>) -> &mut Self {
        self.id = match id {
            Some(id) => Some(id.to_string()),
            None => None,
        };
        self
    }

    pub fn set_to(&mut self, jid: Jid) -> &mut Self {
        self.to = jid;
        self
    }

    pub fn get_to(&self) -> &Jid {
        &self.to
    }

    pub fn get_from(&self) -> Option<&Jid> {
        self.from.as_ref()
    }

    pub fn set_from(&mut self, jid: Option<Jid>) -> &mut Self {
        self.from = jid;

        self
    }

    pub fn to_event(&self) -> Event {
        Event::Stanza(Box::new(StanzaEvent::MessageEvent(Box::new(self.clone()))))
    }
}

impl ToXmlElement for GenericMessage {
    type Error = io::Error;
    fn to_element(&self) -> Result<Element, Self::Error> {
        let mut root = Element::new("message");

        if let Some(message_type) = self.get_type() {
            root.set_attr("type", message_type.to_string());
        }

        if let Some(id) = self.get_type() {
            root.set_attr("id", id.to_string());
        }

        root.set_attr("to", self.get_to().to_string());

        if let Some(from) = self.get_from() {
            root.set_attr("from", from.to_string());
        }

        if let Some(ref childs) = self.childs {
            for child in childs {
                root.append_child(child.clone());
            }
        }

        Ok(root)
    }
}

impl FromXmlElement for GenericMessage {
    type Error = io::Error;
    fn from_element(e: Element) -> Result<Self, Self::Error> {
        // `id` is Optional
        let id = match e.get_attr("id") {
            Some(id) => Some(id.to_string()),
            None => None,
        };

        let message_type = match MessageType::from_str(e.get_attr("type").unwrap_or("")) {
            Ok(t) => Some(t),
            Err(_) => None,
        };

        // `to` is REQUIRED
        let to = match Jid::from_str(e.get_attr("to").unwrap_or("")) {
            Ok(j) => j,
            Err(_) => return Err(io::Error::new(io::ErrorKind::InvalidInput, "to missing")),
        };

        // `from` is OPTIONAL
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

        let childs: Vec<Element> = e.children().cloned().collect();
        Ok(GenericMessage {
               id: id,
               from: from,
               to: to,
               message_type: message_type,
               element: Some(e),
               childs: Some(childs),
           })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PresenceType {
    Available,
}

impl FromStr for PresenceType {
    type Err = io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "available" => Ok(PresenceType::Available),
            _ => Err(io::Error::new(io::ErrorKind::InvalidInput, "Unsupported PresenceType")),
        }
    }
}

impl Into<String> for PresenceType {
    fn into(self) -> String {
        match self {
            PresenceType::Available => "available".to_string(),
        }
    }
}

impl fmt::Display for PresenceType {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", match *self {
            PresenceType::Available => "available".to_string(),
        })
    }
}
// <stanza-kind to='sender' type='error'>
//   [RECOMMENDED to include sender XML here]
//   <error type='error-type'>
//     <defined-condition xmlns='urn:ietf:params:xml:ns:xmpp-stanzas'/>
//     <text xmlns='urn:ietf:params:xml:ns:xmpp-stanzas'
//           xml:lang='langcode'>
//       OPTIONAL descriptive text
//     </text>
//     [OPTIONAL application-specific condition element]
//   </error>
// </stanza-kind>

#[derive(Debug, Clone)]
pub struct StanzaError {
    error_type: StanzaErrorType,
    defined_condition: StanzaDefinedCondition,
    text: Option<Element>,
    application_specific: Option<Element>
}

impl FromXmlElement for StanzaError {
    type Error = io::Error;
    fn from_element(e: Element) -> Result<Self, Self::Error> {
        let error = StanzaError {
            error_type: match e.get_attr("type") {
                Some(t) => match StanzaErrorType::from_str(t) {
                    Ok(t) => t,
                    Err(e) => {
                        return Err(e);
                    }
                },
                None => {
                    return Err(io::Error::new(io::ErrorKind::InvalidInput,
                                              "IqType Error MUST have a type attribute"))
                }
            },
            // MUST contain a child element corresponding to one of the defined stanza error conditions specified below; this element MUST be qualified by the 'urn:ietf:params:xml:ns:xmpp-stanzas' namespace.
            defined_condition: StanzaDefinedCondition::None,

            // MAY contain a <text/> child containing XML character data that describes the error in more detail; this element MUST be qualified by the 'urn:ietf:params:xml:ns:xmpp-stanzas' namespace and SHOULD possess an 'xml:lang' attribute.
            text: e.find((ns::STANZAS, "text")).cloned(),


            application_specific: None
        };

        Ok(error)
    }
}

//TODO: FIX
#[derive(Debug, Clone, PartialEq)]
pub enum StanzaDefinedCondition {
    None
}

#[derive(Debug, Clone, PartialEq)]
pub enum StanzaErrorType {
    Cancel,// -- do not retry (the error is unrecoverable)
    Continue, // -- proceed (the condition was only a warning)
    Modify, // -- retry after changing the data sent
    Auth, // -- retry after providing credentials
    Wait // -- retry after waiting (the error is temporary)
}

impl FromStr for StanzaErrorType {
    type Err = io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "cancel" => Ok(StanzaErrorType::Cancel),
            "continue" => Ok(StanzaErrorType::Continue),
            "modify" => Ok(StanzaErrorType::Modify),
            "auth" => Ok(StanzaErrorType::Auth),
            "wait" => Ok(StanzaErrorType::Wait),
            _ => Err(io::Error::new(io::ErrorKind::InvalidInput, "Unsupported StanzaErrorType")),
        }
    }
}

impl Into<String> for StanzaErrorType {
    fn into(self) -> String {
        match self {
            StanzaErrorType::Cancel => "cancel".to_string(),
            StanzaErrorType::Continue => "continue".to_string(),
            StanzaErrorType::Modify => "modify".to_string(),
            StanzaErrorType::Auth => "auth".to_string(),
            StanzaErrorType::Wait => "wait".to_string()
        }
    }
}

impl fmt::Display for StanzaErrorType {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        let s: String = self.to_string();
        write!(f, "{}", s)
    }
}
