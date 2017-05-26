pub const DOMAIN: &'static str = "example.com";
/// Define a struct to hold general configuration information needed by xmpp
///
#[derive(Clone, PartialEq, Debug)]
pub struct XMPPConfig {
    domain: String,
}

impl Default for XMPPConfig {
    fn default() -> XMPPConfig {
        XMPPConfig { domain: DOMAIN.to_string() }
    }
}
impl XMPPConfig {
    /// Return a fresh instance of an XMPPConfig
    pub fn new() -> XMPPConfig {
        XMPPConfig { ..XMPPConfig::default() }
    }

    /// Define the xmpp domain
    pub fn set_domain(mut self, v: &str) -> Self {
        self.domain = v.to_string();
        self
    }

    /// Return the configured domain
    pub fn get_domain(&self) -> &str {
        &self.domain
    }
}

pub mod ns {
    pub const CLIENT: &'static str = "jabber:client";
    // pub const SERVER: &'static str = "jabber:server";
    pub const STREAM: &'static str = "http://etherx.jabber.org/streams";
    pub const TLS: &'static str = "urn:ietf:params:xml:ns:xmpp-tls";
    pub const SASL: &'static str = "urn:ietf:params:xml:ns:xmpp-sasl";
    pub const BIND: &'static str = "urn:ietf:params:xml:ns:xmpp-bind";
    pub const SESSION: &'static str = "urn:ietf:params:xml:ns:xmpp-session";
    pub const STANZAS: &'static str = "urn:ietf:params:xml:ns:xmpp-stanzas";
    pub const PING: &'static str = "urn:xmpp:ping";
}
