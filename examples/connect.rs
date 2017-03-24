extern crate xmpp_client;
extern crate tokio_core;
extern crate futures;

use std::net::TcpStream;
use tokio_core::reactor::Core;
use futures::stream::Stream;
use futures::sync::mpsc;
use std::thread;

fn main() {

    let (tx, rx) = mpsc::channel(1000);
    let domain = "127.0.0.1";
    let client = xmpp_client::Client::new("user", "pass", "127.0.0.1", "example.com");

    client.connect(tx);

    let mut core = Core::new().expect("Failed to create core");

    let xx = rx.for_each(|s| {

        Ok(())
    });

    core.run(xx).expect("Core failed to run");
}
