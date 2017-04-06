extern crate log;
extern crate xmpp_proto;
extern crate base64;
#[macro_use]extern crate futures;
extern crate tokio_core;
extern crate native_tls;
extern crate tokio_tls;
extern crate tokio_io;
extern crate bytes;

mod client;

pub use client::Client;
