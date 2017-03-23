#![deny(warnings)]
extern crate base64;
extern crate futures;
extern crate tokio_core;
extern crate native_tls;
extern crate tokio_tls;
extern crate tokio_io;
extern crate bytes;

pub mod client;
mod connect;
mod codec;
mod stanza;

pub use connect::ClientMessage;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
