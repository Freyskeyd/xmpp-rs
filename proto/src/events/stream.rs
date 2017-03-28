use events::EventTrait;
use ns;
use events::XMPPConfig;

pub struct OpenStream {
    config: XMPPConfig
}

impl EventTrait for OpenStream {
    type Item = Self;

    fn namespace() -> &'static str {
        ns::STREAM
    }

    fn new(config: &XMPPConfig) -> Self {
        OpenStream {
            config: config.clone()
        }
    }

    fn compute(&self) -> String {
        let ns_xmlns = if self.config.get_instance_type() {
            ns::CLIENT
        } else {
            ns::SERVER
        };

        format!("<stream:stream version='1.0' xmlns:stream='{ns_stream}' to='{to}' xmlns='{ns}'>",
            to=self.config.get_domain(),
            ns_stream=Self::namespace(),
            ns=ns_xmlns)
    }
}

pub struct StartTls {}

impl EventTrait for StartTls {
    type Item = Self;

    fn namespace() -> &'static str {
        ns::TLS
    }

    fn new(_: &XMPPConfig) -> Self {
        StartTls {  }
    }

    fn compute(&self) -> String {
        format!("<starttls xmlns='{ns}' />",ns=Self::namespace())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_namespace() {
        assert!(OpenStream::namespace() == ns::STREAM)
    }

    #[test]
    fn check_compilation() {
        let initial_stream         = "<stream:stream version='1.0' xmlns:stream='http://etherx.jabber.org/streams' to='hey' xmlns='jabber:client'>";
        let initial_stream_example = "<stream:stream version='1.0' xmlns:stream='http://etherx.jabber.org/streams' to='example.com' xmlns='jabber:client'>";

        assert!(OpenStream::new(&XMPPConfig::new().set_domain("hey")).compute() == initial_stream.to_string(), OpenStream::new(&XMPPConfig::new()).compute());
        assert!(OpenStream::new(&XMPPConfig::new()).compute() == initial_stream_example.to_string());
    }
}
