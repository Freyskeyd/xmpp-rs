#![cfg_attr(feature = "lint", allow(unstable_features))]
#![cfg_attr(feature = "lint", feature(plugin))]
#![cfg_attr(feature = "lint", plugin(clippy))]

#![deny(warnings)]

//! Proto

#[macro_use]
extern crate xmpp_derive;
#[macro_use]
extern crate log;
extern crate futures;
extern crate tokio_core;
extern crate native_tls;
extern crate tokio_tls;
extern crate tokio_io;
extern crate bytes;
extern crate base64;
// extern crate elementtree;
extern crate xml;
extern crate uuid;
extern crate circular;
extern crate sasl;
extern crate string_cache;

mod default;
mod config;
pub mod events;
mod jid;
mod stream;
mod codec;
mod stanza;
mod connection;
mod ns;
mod transport;
mod credentials;
mod parser;
mod xmpp_xml;

pub use parser::*;
pub use config::XMPPConfig;
pub use transport::XMPPTransport;
pub use connection::Connection;
pub use connection::ConnectionState;
pub use stream::XMPPStream;
pub use codec::XMPPCodec;
pub use credentials::Credentials;
pub use jid::Jid;
pub use jid::ToJid;
pub use xmpp_xml::{Element, WriteOptions};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
