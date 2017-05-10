use events::IqEvent::BindEvent;
use events::StanzaEvent;
use events::Event;
use events::EventTrait;
use events::ToXmlElement;
use events::GenericIq;
use events::IqType;
use events::FromGeneric;
use events::FromXmlElement;
use jid::Jid;
use std::io;
use std::str::FromStr;
use elementtree::Element;
use ns;

#[derive(Debug, Default, Clone, XmppEvent)]
#[stanza(event = "BindEvent(_)", is="iq", no_transpile)]
pub struct Bind {
    pub generic: GenericIq,
    pub jid: Option<Jid>,
}

impl Bind {
    pub fn new() -> Bind {
        Bind {
            generic: GenericIq::new(&GenericIq::unique_id(), IqType::Get),
            jid: Some(Jid::from_str("").unwrap()),
        }
    }
}

impl FromXmlElement for Bind {
    type Error = io::Error;
    fn from_element(e: Element) -> Result<Bind, Self::Error> {
        let generic = GenericIq::from_element(e).unwrap();
        let body = generic.get_element().unwrap();
        let jid = match body.find("jid") {
            Some(jid) => Jid::from_str(jid.text()).unwrap(),
            None => Jid::from_str("").unwrap(),
        };

        Ok(Bind {
               generic: generic.clone(),
               jid: Some(jid),
           })
    }
}

impl ToXmlElement for Bind {
    type Error = io::Error;
    fn to_element(&self) -> Result<Element, Self::Error> {
        let mut element = self.generic.to_element().unwrap();
        element.append_new_child((ns::BIND, "bind"));

        Ok(element)
    }
}

impl FromGeneric for Bind {
    type Generic = GenericIq;
    type Out = Self;

    fn from_generic(event: &Self::Generic) -> Result<Self::Out, io::Error> {
        let jid = match event.get_element() {
            Some(body) => {
                match body.find("jid") {
                    Some(jid) => Some(Jid::from_str(jid.text()).unwrap()),
                    None => None,
                }
            }
            None => None,
        };

        Ok(Bind {
               generic: event.clone(),
               jid: jid,
           })
    }
}
