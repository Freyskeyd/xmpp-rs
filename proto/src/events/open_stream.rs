use std::str::FromStr;
use elementtree::Element;
use std::string::ParseError;
use ns;
use events::XMPPConfig;

#[derive(Debug, Clone)]
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

        format!("<stream:stream version='1.0' xmlns:stream='{ns_stream}' to='{to}' xmlns='{ns}'>",
                to=to,
                ns_stream=ns::STREAM,
                ns=self.xmlns)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_compilation() {
        let initial_stream         = "<stream:stream version='1.0' xmlns:stream='http://etherx.jabber.org/streams' to='hey' xmlns='jabber:client'>";
        let initial_stream_example = "<stream:stream version='1.0' xmlns:stream='http://etherx.jabber.org/streams' to='example.com' xmlns='jabber:client'>";

        assert!(OpenStream::new(&XMPPConfig::new().set_domain("hey")).to_string() == initial_stream.to_string(), OpenStream::new(&XMPPConfig::new()).to_string());
        assert!(OpenStream::new(&XMPPConfig::new()).to_string() == initial_stream_example.to_string());
    }
}
