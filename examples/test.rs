extern crate xmpp;
extern crate futures;
extern crate futures_cpupool;
extern crate tokio_core;

use xmpp::connect_client;
use xmpp::ClientMessage;
use futures::sync::mpsc;
use futures::{Sink, Stream};
use std::thread;
use tokio_core::reactor::Core;

fn main() {
    let (tx, rx) = mpsc::channel(1000);
    let mut core = Core::new().expect("Failed to create core");
    thread::spawn(move || {
        connect_client(tx, |_| {
            None
        });
    });

    let xx = rx.for_each(|(_, mut in_tx)| {
        let _ = in_tx.start_send(ClientMessage("<iq from='alice@example.com' to='example.com' id='c2s1' type='get'><ping xmlns='urn:xmpp:ping'/></iq>".to_string()));

        Ok(())
    });

    core.run(xx).expect("Core failed to run");
}

