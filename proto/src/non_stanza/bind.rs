use uuid::Uuid;
use xmpp_xml::Element;

use crate::{ns, FromXmlElement, NonStanza, Packet, ToXmlElement};

#[derive(Default, Debug, Clone)]
pub struct Bind {
    resource: Option<String>,
}

impl From<Bind> for Packet {
    fn from(s: Bind) -> Self {
        NonStanza::Bind(s).into()
    }
}

impl FromXmlElement for Bind {
    type Error = std::io::Error;
    fn from_element(e: &Element) -> Result<Self, Self::Error> {
        let p = Self {
            resource: e.find((ns::BIND, "resource")).map(|e| e.text().to_owned()),
        };

        Ok(p)
    }
}

impl ToXmlElement for Bind {
    type Error = std::io::Error;
    fn to_element(&self) -> Result<Element, std::io::Error> {
        let mut bind = Element::new((ns::BIND, "bind"));
        bind.append_new_child((ns::BIND, "jid"))
            .set_text(format!("SOME@localhost/{}", self.resource.clone().unwrap_or_else(|| Uuid::new_v4().to_string())));

        Ok(bind)
    }
}
