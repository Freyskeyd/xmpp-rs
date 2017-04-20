use elementtree::{WriteOptions, Element};
use events::IqEvent::PingEvent;
use events::*;
use ns;
use jid::{Jid, ToJid};
use std::io;
use std::str::FromStr;
use std::string::ParseError;
use std::str;

#[derive(Debug, Clone, XmppEvent)]
#[stanza(event="PingEvent(_)", is="iq", no_transpile)]
pub struct Ping {
    pub generic: GenericIq
}

impl Ping {
    pub fn new<F: ToJid + ?Sized, T: ToJid + ?Sized>(from: &F, to: &T) -> Ping {
        let mut generic = GenericIq::new(&GenericIq::unique_id(), IqType::Get);
        let _ = generic.set_from(Some(from))
                .unwrap()
                .set_to(Some(to));
        Ping {
            generic: generic
        }
    }
}

impl FromStr for Ping {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let generic = GenericIq::from_str(s).unwrap();

        Ok(Ping { generic: generic})
    }
}

impl Default for Ping {
    fn default() -> Ping {
        Ping {
            generic: GenericIq::new(&GenericIq::unique_id(), IqType::Get)
        }
    }
}

impl ToString for Ping {
    fn to_string(&self) -> String {
        let mut out:Vec<u8> = Vec::new();
        let mut root = Element::new("iq");
        let options = WriteOptions::new()
            .set_xml_prolog(None);

        root.set_attr("type", self.generic.get_type().to_string());
        root.set_attr("id", self.generic.get_id());

        if let Some(ref from) = self.generic.get_from() {
            root.set_attr("from", from.to_string());
        };

        if let Some(ref to) = self.generic.get_to() {
            root.set_attr("to", to.to_string());
        };

        match self.generic.get_type() {
            IqType::Result => {
            },
            _ => {
                root.append_new_child((ns::PING, "ping"));
            }
        };
        root.to_writer_with_options(&mut out, options).unwrap();
        String::from_utf8(out).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[ignore]
    fn test() {
        let p = Ping::new("hello@example.com", "hey");

        assert_eq!(p.to_string(), "<iq id=\"c2s1\" from=\"hello@example.com\" type=\"get\" to=\"hey\"><ping xmlns=\"urn:xmpp:ping\" /></iq>")
    }
}
