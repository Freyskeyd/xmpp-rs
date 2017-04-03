#[macro_use]extern crate log;
#[macro_use]extern crate futures;
extern crate tokio_core;
extern crate native_tls;
extern crate tokio_tls;
extern crate tokio_io;
extern crate bytes;
extern crate base64;
#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate elementtree;


pub mod default;
pub mod config;
pub mod events;
pub mod jid;
pub mod stream;
pub mod codec;
pub mod stanza;
pub mod connection;
pub mod ns;
pub mod transport;
pub mod credentials;
pub mod parser;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
