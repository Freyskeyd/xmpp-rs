//! xmpp-client
//!
//! This library offers an xmpp client implementation over tcp/tls protocol.
//! It's deeply use tokio-io and futures-rs library to interact with an xmpp-server
//!
//! This client didn't create the TcpStream itself, you need to provide a TcpStream or a TlsStream.
//!
//! ## Listen for incoming message
//!
//! ```rust,no_run
//! extern crate xmpp_client;
//! extern crate tokio_core;
//! extern crate futures;
//! 
//! use tokio_core::reactor::Core;
//! use futures::Future;
//! use futures::Stream;
//! use xmpp_client::{XMPPConfig, Credentials, Jid};
//! use tokio_core::net::TcpStream;
//! use xmpp_client::events::Event::Stanza;
//! use xmpp_client::events::StanzaEvent::{MessageEvent};
//! 
//! fn main() {
//!     let mut core = Core::new().unwrap();
//! 
//!     let handle = core.handle();
//!     let addr = "127.0.0.1:5222".parse().unwrap();
//! 
//!     // Define an XMPPConfig and set the domain to `example.com`
//!     let config = XMPPConfig::new()
//!           .set_domain("example.com");
//!
//!     // Define client credentials
//!     let credentials = Credentials {
//!         jid: Jid::from_full_jid("alice@example.com"),
//!         password: String::from("test")
//!     };
//!
//!     core.run(
//!         // Create the TcpStream and then launch our client
//!         TcpStream::connect(&addr, &handle).and_then(|stream| {
//!             xmpp_client::Client::connect(stream, config, Some(credentials))
//!         }).and_then(|mut client| {
//!             // Define and trigger the first presence for our connected client
//!             handle.spawn(client.send_presence().then(move|_| {
//!                 Ok(())
//!             }));
//!
//!             // Create a handler on the stream to collect every message
//!             client.handle().and_then(move |stream| {
//!                 stream.for_each(move |m| {
//!                     match m {
//!                         Stanza(event) => match *event {
//!                             MessageEvent(_) => {
//!                                 println!("New message");
//!                             },
//!                             _ => {}
//!                         },
//!                         _ => {}
//!                     }
//!                     Ok(())
//!                 })
//!             })
//!         })).unwrap();
//! }
//!
//! ```

#![deny(warnings, missing_docs)]
extern crate xmpp_proto;
extern crate futures;
extern crate native_tls;
extern crate tokio_core;
extern crate tokio_tls;
extern crate tokio_io;
// extern crate xml;

mod client;

pub use xmpp_proto::*;
pub use client::Client;
