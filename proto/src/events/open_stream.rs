use std::str::FromStr;
use super::Event;
use super::NonStanzaEvent;
use super::EventTrait;
use elementtree::Element;
use std::string::ParseError;
use ns;
use config::XMPPConfig;

#[derive(Debug, Clone, XmppEvent)]
#[non_stanza(event = "NonStanzaEvent::OpenStreamEvent(_)")]
pub struct OpenStream {
    config: XMPPConfig,
    pub to: Option<String>,
    pub xmlns: String,
}

impl OpenStream {
    pub fn new(config: &XMPPConfig) -> OpenStream {
        OpenStream {
            config: config.clone(),
            to: Some(config.get_domain().to_string()),
            xmlns: ns::CLIENT.to_string(),
        }
    }
}

impl FromStr for OpenStream {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let s = format!("{}</stream:stream>", s);
        let root = Element::from_reader(s.as_bytes()).unwrap();

        let to = match root.get_attr("to") {
            Some(to) => Some(to.to_string()),
            None => None
        };

        let xmlns = match root.get_namespace_prefix("jabber:client") {
            Some(_) => ns::CLIENT,
            None => ns::SERVER
        };

        Ok(OpenStream {
            config: XMPPConfig::new(),
            to: to,
            xmlns: xmlns.to_string(),
        })
    }
}

impl ToString for OpenStream {
    fn to_string(&self) -> String {
        let to = match self.to {
            Some(ref t) => t.as_str(),
            None => ""
        };

        format!("<?xml version='1.0'?><stream:stream version='1.0' xmlns:stream='{ns_stream}' to='{to}' xmlns='{ns}'>",
                to=to,
                ns_stream=ns::STREAM,
                ns=self.xmlns)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use events::*;
    use config::*;
    use events::interface::EventTrait;

    #[test]
    fn check_compilation() {
        let initial_stream         = "<?xml version='1.0'?><stream:stream version='1.0' xmlns:stream='http://etherx.jabber.org/streams' to='hey' xmlns='jabber:client'>";
        let initial_stream_example = "<?xml version='1.0'?><stream:stream version='1.0' xmlns:stream='http://etherx.jabber.org/streams' to='example.com' xmlns='jabber:client'>";

        assert!(OpenStream::new(&XMPPConfig::new().set_domain("hey")).to_string() == initial_stream.to_string(), OpenStream::new(&XMPPConfig::new()).to_string());
        assert!(OpenStream::new(&XMPPConfig::new()).to_string() == initial_stream_example.to_string());
    }

    #[test]
    fn compile() {
        let o = OpenStream::new(&XMPPConfig::new().set_domain("hey"));

        let e = o.to_event();

        assert!(!e.is_message());
        assert!(!e.is_iq());
        assert!(e.is_non_stanza());

        // assert!(e.is::)
        match o.to_event() {
            Event::NonStanza(non_stanza, _) => match *non_stanza {
                NonStanzaEvent::OpenStreamEvent(_) => {
                    assert!(true);
                },
                _ => panic!("")
            },
            _ => panic!("")
        }
    }
}
