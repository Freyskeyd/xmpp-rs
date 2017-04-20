use events::{PresenceType, Event, EventTrait, StanzaEvent};
use elementtree::{WriteOptions, Element};
use std::io::{self};
use std::str::FromStr;
use jid::{Jid, ToJid};

#[derive(Debug, Clone, XmppEvent)]
#[stanza(is="presence")]
pub struct Presence {
    to: Option<Jid>,
    from: Option<Jid>,
    presence_type: Option<PresenceType>,

}
impl Presence {
    pub fn new() -> Presence {
        Presence {
            to: None,
            from: None,
            presence_type: None
        }
    }

    pub fn set_type<'a>(&'a mut self, presence_type: Option<PresenceType>) -> Result<&'a mut Self, io::Error> {
        self.presence_type = presence_type;
        Ok(self)
    }

    pub fn get_type(&self) -> Option<&PresenceType> {
        self.presence_type.as_ref()
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
}

impl ToString for Presence {
    fn to_string(&self) -> String {
        let mut out:Vec<u8> = Vec::new();
        let mut root = Element::new("presence");
        let options = WriteOptions::new()
            .set_xml_prolog(None);

        if let Some(t) = self.get_type() {
            root.set_attr("type", t.to_string());
        }

        root.to_writer_with_options(&mut out, options).unwrap();
        String::from_utf8(out).unwrap()
    }
}

impl FromStr for Presence {
    type Err = io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let root = match Element::from_reader(s.as_bytes()) {
            Ok(r) => r,
            Err(e) => return Err(io::Error::new(io::ErrorKind::InvalidInput, e))
        };

        let presence_type = match root.get_attr("type") {
            Some(t) => match PresenceType::from_str(t) {
                Ok(t) => Some(t),
                Err(e) => return Err(io::Error::new(io::ErrorKind::InvalidInput, e))
            },
            None => None
        };

        let to = match Jid::from_str(root.get_attr("to").unwrap_or("")) {
            Ok(j) => Some(j),
            Err(_) => None
        };

        let from = match Jid::from_str(root.get_attr("from").unwrap_or("")) {
            Ok(j) => Some(j),
            Err(_) => None
        };

        Ok(Presence {
            from: from,
            to: to,
            presence_type: presence_type,
        })
    }
}

