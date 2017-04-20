use events::IqEvent::BindEvent;
use events::StanzaEvent;
use events::Event;
use events::EventTrait;
use events::GenericIq;
use events::IqType;
use events::FromGeneric;
use jid::{Jid, ToJid};
use std::io;
use std::str::FromStr;
use std::string::ParseError;

#[derive(Debug, Default, Clone, XmppEvent)]
#[stanza(event = "BindEvent(_)", is="iq", no_transpile)]
pub struct Bind {
    pub generic: GenericIq,
    pub jid: Option<Jid>
}

impl Bind {
    pub fn new() -> Bind {
        Bind {
            generic: GenericIq::new(&GenericIq::unique_id(), IqType::Get),
            jid: Some(Jid::from_str("").unwrap())
        }
    }
}

impl FromGeneric for Bind {
    type Generic = GenericIq;
    type Out = Self;

    fn from_generic<'a>(event: &'a Self::Generic) -> Result<Self::Out, io::Error> {
        let jid = match event.get_element() {
            Some(body) => match body.find("jid") {
                Some(jid) => Some(Jid::from_str(jid.text()).unwrap()),
                None => None
            },
            None => None
        };

        Ok(Bind {
            generic: event.clone(),
            jid: jid
        })
    }
}
impl FromStr for Bind {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let generic = GenericIq::from_str(s).unwrap();

        let body = generic.get_element().unwrap();
        let jid = match body.find("jid") {
            Some(jid) => Jid::from_str(jid.text()).unwrap(),
            None => Jid::from_str("").unwrap()
        };

        Ok(Bind {
            generic: generic.clone(),
            jid: Some(jid),
        })
    }
}

impl ToString for Bind {
    fn to_string(&self) -> String {
        format!("<iq type='{bind_type}' id='{id}'><bind xmlns='urn:ietf:params:xml:ns:xmpp-bind'/></iq>", id=self.get_id(), bind_type=self.get_type())
    }
}


