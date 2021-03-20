#![cfg_attr(feature = "lint", allow(unstable_features))]
#![cfg_attr(feature = "lint", feature(plugin))]
#![cfg_attr(feature = "lint", plugin(clippy))]

#![deny(warnings)]

//! XMPP Proto is the common and core structs and algorithm to interact between entities.
//!
//! It offer everything to connect, manager, share stanza over a TcpStream.

#[macro_use]
extern crate log;
extern crate futures;
extern crate tokio_core;
extern crate native_tls;
extern crate tokio_tls;
extern crate tokio_io;
extern crate bytes;
extern crate base64;
extern crate xml;
extern crate uuid;
extern crate circular;
extern crate sasl;
extern crate xmpp_events;
extern crate xmpp_xml;
extern crate xmpp_config;
extern crate xmpp_credentials;
extern crate jid;

mod stream;
mod codec;
mod connection;
mod transport;
mod parser;

pub use parser::XmppParser;
pub use transport::XMPPTransport;
pub use connection::Connection;
pub use connection::ConnectionState;
pub use stream::XMPPStream;
pub use codec::XMPPCodec;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
