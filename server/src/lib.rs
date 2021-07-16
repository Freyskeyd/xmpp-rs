#[macro_use]
extern crate lazy_static;

use crate::config::Settings;
use crate::{authentication::AuthenticationManager, sessions::manager::SessionManager};
use actix::{Actor, Recipient};
use log::info;
use router::Router;
use std::collections::HashMap;

mod authentication;
mod config;
mod listeners;
mod messages;
mod packet;
mod parser;
mod router;
mod sessions;
#[cfg(test)]
mod tests;

pub use messages::AuthenticationRequest;

lazy_static! {
    static ref CONFIG: Settings = Settings::new().unwrap();
}

pub trait Service {
    type Config;

    fn create_with_config(config: &Self::Config) -> Self;
}

pub struct Server {}

impl Server {
    pub fn build() -> ServerBuilder {
        ServerBuilder::default()
    }
}

#[derive(Default)]
pub struct ServerBuilder {
    cert: Option<String>,
    keys: Option<String>,
    authenticators: HashMap<String, Recipient<AuthenticationRequest>>,
}

impl ServerBuilder {
    pub fn cert<T: Into<String>>(mut self, cert: T) -> Self {
        self.cert = Some(cert.into());

        self
    }

    pub fn keys<T: Into<String>>(mut self, keys: T) -> Self {
        self.keys = Some(keys.into());

        self
    }

    pub fn add_authenticator(mut self, authenticator_name: &str, authenticator: Recipient<AuthenticationRequest>) -> Self {
        self.authenticators.insert(authenticator_name.into(), authenticator);
        self
    }

    pub async fn launch(self) -> std::io::Result<()> {
        println!("CONFIG: {:?}", *CONFIG);
        SessionManager::new().start();
        AuthenticationManager::default().register(&self.authenticators).start();
        // Starting systemd
        // Starting hooks
        // Starting clustering
        // Starting translation
        // Starting access permissions
        // Starting ctl
        // Starting commands
        // Starting admin
        // Starting Router
        let router = Router::new().start();
        // Starting all listener (tcp, ws)
        for listener_cfg in CONFIG.listeners.iter() {
            match listener_cfg {
                config::ListenerConfig::Tcp(tcp_config) => {
                    let _tcp_listener = crate::listeners::tcp::listener::TcpListener::start(tcp_config, router.clone());
                }
            }
        }
        // let _ws_listener = crate::listeners::ws::ws_listener();
        // Starting pkix
        // Starting ACL
        // Starting Shaper
        // Starting DB
        // Starting Backend
        // Starting Sql
        // Starting IQ
        // Starting router multicast
        // Starting local
        // Starting Session Manager
        // Starting s2s_in
        // Starting s2s_out
        // Starting s2s
        // Starting service
        // Starting captcha
        // Starting ext_mod
        // Starting acme
        // Starting auth
        // Starting oauth

        // Start API

        tokio::signal::ctrl_c().await.unwrap();
        info!("Ctrl-C received, shutting down");

        Ok(())
    }
}

pub trait Authenticator: actix::Actor {
    // type Future;
    // type Error;

    // fn poll_ready(&mut self, cx: Context<'_>) -> Poll<Result<(), Self::Error>>;
    // fn authenticate(&mut self, request: AuthenticationRequest) -> Self::Future;
}
