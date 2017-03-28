use default;

#[derive(Clone, PartialEq, Debug)]
pub struct XMPPConfig {
    domain: String,
    instance_type: bool
}

impl Default for XMPPConfig {
    fn default() -> XMPPConfig {
        XMPPConfig {
            domain: default::DOMAIN.to_string(),
            instance_type: true
        }
    }
}
impl XMPPConfig {
    pub fn new() -> XMPPConfig {
        XMPPConfig { ..XMPPConfig::default() }
    }

    pub fn set_domain(mut self, v: &str) -> Self {
        self.domain = v.to_string();
        self
    }

    pub fn get_domain(&self) -> &str {
        &self.domain
    }

    pub fn get_instance_type(&self) -> bool {
        self.instance_type
    }
}
