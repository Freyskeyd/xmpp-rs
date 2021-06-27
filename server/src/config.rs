use config::{Config, ConfigError, File};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub(crate) struct Settings {
    pub(crate) listeners: Vec<ListenerConfig>,
    pub(crate) authenticators: Vec<String>,
    pub(crate) vhosts: HashMap<String, VhostConfig>,
}

//TODO: Rethink how to access per vhost config and config inheritance
impl Settings {
    pub(crate) fn new() -> Result<Self, ConfigError> {
        let mut s = Config::default();

        s.merge(File::with_name("config/default"))?;

        let default_authenticators = s.get::<Vec<String>>("authenticators")?;

        let vhosts = s.get_table("vhosts")?;

        vhosts.iter().for_each(|(vhost, _)| {
            let target = format!("vhosts.{}.authenticators", vhost);

            let _ = s.set_default(&target, default_authenticators.clone());
        });

        println!("vhosts : {:?}", s.get::<HashMap<String, VhostConfig>>("vhosts"));
        let r = s.try_into();

        r
    }
}

#[derive(Debug, Deserialize)]
pub(crate) enum ListenerConfig {
    Tcp(TcpListenerConfig),
    // Ws,
    // Udp
}

#[derive(Debug, Deserialize)]
pub(crate) struct TcpListenerConfig {
    pub(crate) port: i32,
    pub(crate) ip: String,
    pub(crate) starttls: StartTLSConfig,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Cert {
    pub(crate) cert_path: String,
    pub(crate) key_path: String,
}

#[derive(Debug, Deserialize)]
pub(crate) enum StartTLSConfig {
    Unavailable,
    Available(Cert),
    Required(Cert),
}

impl std::fmt::Display for StartTLSConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StartTLSConfig::Unavailable => write!(f, "starttls unavailable"),
            StartTLSConfig::Available(_) => write!(f, "starttls available"),
            StartTLSConfig::Required(_) => write!(f, "starttls required"),
        }
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct VhostConfig {
    pub(crate) authenticators: Vec<String>,
}
