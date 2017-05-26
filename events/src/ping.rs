use xmpp_xml::Element;
use super::IqEvent::PingEvent;
use super::*;
use xmpp_config::ns;
use xmpp_jid::Jid;
use std::io;
use std::str;

#[derive(Debug, Clone, XmppEvent)]
#[stanza(event="PingEvent(_)", is="iq", no_transpile)]
pub struct Ping {
    pub generic: GenericIq,
}

impl Ping {
    pub fn new(from: Jid, to: Jid) -> Ping {
        let mut generic = GenericIq::new(&GenericIq::unique_id(), IqType::Get);
        generic.set_from(Some(from));
        generic.set_to(Some(to));

        Ping { generic: generic }
    }
}

impl FromXmlElement for Ping {
    type Error = io::Error;
    fn from_element(e: Element) -> Result<Self, Self::Error> {
        let generic = match GenericIq::from_element(e) {
            Ok(g) => g,
            Err(e) => {
                return Err(io::Error::new(io::ErrorKind::InvalidInput, e));
            }
        };

        match generic.get_type() {
            IqType::Get => {
                if generic
                       .get_element()
                       .unwrap()
                       .find((ns::PING, "ping"))
                       .is_none() {
                    return Err(io::Error::new(io::ErrorKind::InvalidInput, "Ping element not found"));
                }
            }
            IqType::Result => {
                if generic.get_element().unwrap().child_count() > 0 {
                    return Err(io::Error::new(io::ErrorKind::InvalidInput, "Ping result can't have body"));
                }
            }
            _ => {}
        }

        Ok(Ping { generic: generic })
    }
}

impl ToXmlElement for Ping {
    type Error = io::Error;
    fn to_element(&self) -> Result<Element, Self::Error> {
        let mut element = self.generic.to_element().unwrap();

        match self.generic.get_type() {
            IqType::Result => {}
            _ => {
                element.append_new_child((ns::PING, "ping"));
            }
        };

        Ok(element)
    }
}

impl Default for Ping {
    fn default() -> Ping {
        Ping { generic: GenericIq::new(&GenericIq::unique_id(), IqType::Get) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn s2c_ping() {
        let test_str = Element::from_reader(r#"<iq from='capulet.lit' to='juliet@capulet.lit/balcony' id='s2c1' type='get'><ping xmlns='urn:xmpp:ping'/></iq>"#.as_bytes());

        assert!(test_str.is_ok());
        let e = test_str.unwrap();
        let e = Ping::from_element(e).unwrap();
        assert!(e.get_from() == Some(&Jid::from_full_jid("capulet.lit")));
        assert!(e.get_to() == Some(&Jid::from_full_jid("juliet@capulet.lit/balcony")));
        assert!(e.get_id() == "s2c1");
        assert!(e.get_type() == IqType::Get);
    }

    #[test]
    fn s2c_pong() {
        let test_str = Element::from_reader(r#"<iq from='juliet@capulet.lit/balcony' to='capulet.lit' id='s2c1' type='result'/>"#.as_bytes());


        assert!(test_str.is_ok());
        let e = test_str.unwrap();
        let e = Ping::from_element(e).unwrap();
        assert!(e.get_from() == Some(&Jid::from_full_jid("juliet@capulet.lit/balcony")));
        assert!(e.get_to() == Some(&Jid::from_full_jid("capulet.lit")));
        assert!(e.get_id() == "s2c1");
        assert!(e.get_type() == IqType::Result);
    }

    #[test]
    fn s2c_error() {
        let test_str = Element::from_reader(r#"
        <iq from='juliet@capulet.lit/balcony'
            to='capulet.lit'
            id='s2c1'
            type='error'>
            <ping xmlns='urn:xmpp:ping'/>
            <error type='cancel'>
                <service-unavailable
                    xmlns='urn:ietf:params:xml:ns:xmpp-stanzas'/>
            </error>
        </iq>"#
                                                    .as_bytes());

        assert!(test_str.is_ok());
        let e = test_str.unwrap();
        let e = Ping::from_element(e).unwrap();

        assert!(e.get_from() == Some(&Jid::from_full_jid("juliet@capulet.lit/balcony")));
        assert!(e.get_to() == Some(&Jid::from_full_jid("capulet.lit")));
        assert!(e.get_id() == "s2c1");
        assert!(e.get_type() == IqType::Error);
    }
}
