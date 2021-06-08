use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
pub(crate) struct Settings {
    pub(crate) listeners: Vec<ListenerConfig>,
}

impl Settings {
    pub(crate) fn new() -> Result<Self, ConfigError> {
        let mut s = Config::default();

        s.merge(File::with_name("config/default"))?;

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
