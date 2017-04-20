#![allow(dead_code)]
use std::fmt;
use uuid::Uuid;
use std::io::{self};
use elementtree::{WriteOptions, Element};
use std::string::ToString;
use std::str::FromStr;
use jid::{Jid, ToJid};
use events::IqType;
use events::Event;
use events::StanzaEvent;
use events::IqEvent;

#[derive(Debug, Clone)]
pub struct GenericIq {
    id: String,
    iq_type: IqType,
    to: Option<Jid>,
    from: Option<Jid>,
    element: Option<Element>
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
            element: None
        }
    }

    pub fn unique_id() -> String {
        Uuid::new_v4().to_string()
    }

    pub fn get_element<'a>(&'a self) -> Option<&'a Element> {
        self.element.as_ref()
    }

    pub fn set_type<'a>(&'a mut self, iq_type: IqType) -> Result<&'a mut Self, io::Error> {
        self.iq_type = iq_type;
        Ok(self)
    }

    pub fn get_type(&self) -> IqType {
        self.iq_type
    }

    pub fn get_id(&self) -> &str {
        self.id.as_ref()
    }

    pub fn set_id<'a, T: ToString + ?Sized>(&'a mut self, id: &T) -> &'a mut Self {
        self.id = id.to_string();
        self
    }

    pub fn set_to<'a, T: ToJid + ?Sized>(&'a mut self, jid: Option<&T>) -> Result<&'a mut Self, io::Error> {
        self.to = match jid.to_jid() {
            Ok(jid) => Some(jid),
            Err(e) => return Err(e)
        };
        Ok(self)
    }

    pub fn get_to(&self) -> Option<&Jid> {
        self.to.as_ref()
    }

    pub fn set_from<'a, T: ToJid + ?Sized>(&'a mut self, jid: Option<&T>) -> Result<&'a mut Self, io::Error> {
        self.from = match jid.to_jid() {
            Ok(jid) => Some(jid),
            Err(e) => return Err(e)
        };
        Ok(self)
    }

    pub fn get_from(&self) -> Option<&Jid> {
        self.from.as_ref()
    }

    pub fn to_event(&self) -> Event {
        Event::Stanza(Box::new(StanzaEvent::IqEvent(Box::new(IqEvent::GenericEvent(self.clone())))), self.to_string())
    }
}

impl FromStr for GenericIq {
    type Err = io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let root = match Element::from_reader(s.as_bytes()) {
            Ok(r) => r,
            Err(e) => return Err(io::Error::new(io::ErrorKind::InvalidInput, e))
        };

        // `id` is REQUIRED
        let id = match root.get_attr("id") {
            Some(id) => id.to_string(),
            None => return Err(io::Error::new(io::ErrorKind::InvalidInput, "ID is required"))
        };

        let iq_type = match IqType::from_str(root.get_attr("type").unwrap_or("")) {
            Ok(t) => t,
            Err(e) => return Err(e)
        };

        let to = match Jid::from_str(root.get_attr("to").unwrap_or("")) {
            Ok(j) => Some(j),
            Err(_) => None
        };

        let from = match Jid::from_str(root.get_attr("from").unwrap_or("")) {
            Ok(j) => Some(j),
            Err(_) => None
        };

        Ok(GenericIq {
            id: id,
            iq_type: iq_type,
            from: from,
            to: to,
            element: Some(root)
        })
    }
}

impl ToString for GenericIq {
    fn to_string(&self) -> String {
        let mut out:Vec<u8> = Vec::new();
        let mut root = Element::new("iq");
        let options = WriteOptions::new()
            .set_xml_prolog(None);

        root.set_attr("type", self.iq_type.to_string());
        root.set_attr("id", self.id.to_string());

        if let Some(to) = self.get_to() {
            root.set_attr("to", to.to_string());
        }

        if let Some(from) = self.get_from() {
            root.set_attr("from", from.to_string());
        }

        root.to_writer_with_options(&mut out, options).unwrap();
        String::from_utf8(out).unwrap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MessageType {
    Error
}

impl FromStr for MessageType {
    type Err = io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "error" => Ok(MessageType::Error),
            _ => Err(io::Error::new(io::ErrorKind::InvalidInput, "Unsupported MessageType"))
        }
    }
}

impl Into<String> for MessageType {
    fn into(self) -> String {
        match self {
            MessageType::Error => "error".to_string(),
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
        })
    }
}


#[derive(Debug, Clone)]
pub struct GenericMessage {
    id: Option<String>,
    to: Jid,
    from: Option<Jid>,
    message_type: Option<MessageType>,
    element: Option<Element>
}

impl FromStr for GenericMessage {
    type Err = io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let root = match Element::from_reader(s.as_bytes()) {
            Ok(r) => r,
            Err(e) => return Err(io::Error::new(io::ErrorKind::InvalidInput, e))
        };

        // `id` is Optional
        let id = match root.get_attr("id") {
            Some(id) => Some(id.to_string()),
            None => None
        };

        let message_type = match MessageType::from_str(root.get_attr("type").unwrap_or("")) {
            Ok(t) => Some(t),
            Err(_) => None
        };

        // `to` is REQUIRED
        let to = match Jid::from_str(root.get_attr("to").unwrap_or("")) {
            Ok(j) => j,
            Err(_) => return Err(io::Error::new(io::ErrorKind::InvalidInput, "to missing"))
        };

        // `from` is OPTIONAL
        let from = match Jid::from_str(root.get_attr("from").unwrap_or("")) {
            Ok(j) => Some(j),
            Err(_) => None
        };

        Ok(GenericMessage {
            id: id,
            from: from,
            to: to,
            message_type: message_type,
            element: Some(root)
        })
    }
}

impl GenericMessage {
    pub fn new<T: ToJid + ?Sized>(to: &T) -> GenericMessage {
        GenericMessage {
            id: None,
            to: to.to_jid().unwrap(),
            from: None,
            message_type: None,
            element: None
        }
    }

    pub fn set_type<'a>(&'a mut self, message_type: Option<MessageType>) -> Result<&'a mut Self, io::Error> {
        self.message_type = message_type;
        Ok(self)
    }

    pub fn get_type<'a>(&'a self) -> Option<&'a MessageType> {
        self.message_type.as_ref()
    }

    pub fn get_id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    pub fn set_id<'a, T: ToString>(&'a mut self, id: Option<T>) -> &'a mut Self {
        self.id = match id {
            Some(id) => Some(id.to_string()),
            None => None
        };
        self
    }

    pub fn set_to<'a, T: ToJid + ?Sized>(&'a mut self, jid: &T) -> Result<&'a mut Self, io::Error> {
        self.to = match jid.to_jid() {
            Ok(jid) => jid,
            Err(e) => return Err(e)
        };
        Ok(self)
    }

    pub fn get_to(&self) -> &Jid {
        &self.to
    }

    pub fn get_from(&self) -> Option<&Jid> {
        self.from.as_ref()
    }

    pub fn set_from<'a, T: ToJid + ?Sized>(&'a mut self, jid: Option<&T>) -> Result<&'a mut Self, io::Error> {
        self.from = match jid {
            Some(jid) => match jid.to_jid() {
                Ok(jid) => Some(jid),
                Err(e) => return Err(e)
            },
            None => None
        };

        Ok(self)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PresenceType {
    Available
}

impl FromStr for PresenceType {
    type Err = io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "available" => Ok(PresenceType::Available),
            _ => Err(io::Error::new(io::ErrorKind::InvalidInput, "Unsupported PresenceType"))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generic_1() {
        let g = Generic::from_str("<iq type=\"get\" id=\"1\" />");

        match g {
            Ok(g) => {
                assert!(g.id == "1");
                assert!(g.iq_type == IqType::Get);
            },
            Err(e) => panic!(e)
        }
    }
}
