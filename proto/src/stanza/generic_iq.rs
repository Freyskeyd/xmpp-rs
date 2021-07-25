use super::Stanza;
use crate::{FromXmlElement, Jid, Packet, ToXmlElement};

use std::fmt;
use std::io;
use std::str::FromStr;
use uuid::Uuid;
use xmpp_xml::Element;

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

impl From<IqType> for String {
    fn from(iq_type: IqType) -> String {
        match iq_type {
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
            iq_type,
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
impl ToXmlElement for GenericIq {
    type Error = io::Error;

    fn to_element(&self) -> Result<Element, Self::Error> {
        Ok(self.element.clone().unwrap())
    }
}

impl FromXmlElement for GenericIq {
    type Error = io::Error;
    fn from_element(e: &Element) -> Result<Self, io::Error> {
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
                if e.find("{jabber:client}error").is_none() {
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
