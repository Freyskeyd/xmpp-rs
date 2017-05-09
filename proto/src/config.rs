use default;

/// Define a struct to hold general configuration information needed by xmpp
///
#[derive(Clone, PartialEq, Debug)]
pub struct XMPPConfig {
    domain: String,
}

impl Default for XMPPConfig {
    fn default() -> XMPPConfig {
        XMPPConfig { domain: default::DOMAIN.to_string() }
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
