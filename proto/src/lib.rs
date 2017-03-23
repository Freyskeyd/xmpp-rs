extern crate base64;
extern crate futures;
extern crate tokio_core;
extern crate native_tls;
extern crate tokio_tls;
extern crate tokio_io;
extern crate bytes;


pub mod connect;
pub mod codec;
pub mod stanza;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
