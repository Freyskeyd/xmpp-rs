use actix::Actor;
use log::info;
use router::Router;
use std::path::Path;

use crate::{authentication::AuthenticationManager, sessions::manager::SessionManager};

mod authentication;
mod listeners;
mod parser;
mod router;
mod sessions;
#[cfg(test)]
mod tests;

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

    pub async fn launch(self) -> std::io::Result<()> {
        SessionManager::new().start();
        AuthenticationManager::default().start();
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
        let _tcp_listener = crate::listeners::tcp::listener::TcpListener::start("", router, Path::new(&self.cert.unwrap()), Path::new(&self.keys.unwrap()));
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

// /// Manage listeners on a node
// pub struct Listeners {}
// /// Listen for TCP on a node
// pub struct TcpListener {}
// /// Listen for WS on a node
// pub struct WsListener {}
// /// Hold a TCP session on a node
// pub struct TcpSession {}
// /// Hold a WS session on a node
// pub struct WsSession {}
// pub trait PacketReceiver {}
