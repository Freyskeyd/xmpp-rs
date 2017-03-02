extern crate xmpp as line;
extern crate futures;
extern crate tokio_core;
extern crate tokio_proto;
extern crate tokio_service;
extern crate byteorder;

use std::env;
use tokio_core::reactor::Core;
use futures::{Future};
use tokio_service::{Service};
use std::{str};
use std::net::SocketAddr;
use line::XmppService;

fn main() {
    // The builder requires a protocol and an address
    // let server = TcpServer::new(LineProto, addr);
    let addr = env::args().nth(1).unwrap_or("127.0.0.1:5222".to_string());
    let addr = addr.parse::<SocketAddr>().unwrap();

    // Create the event loop and initiate the connection to the remote server
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let tcp = line::XmppClient::connect(&addr, &handle)
        .and_then(|client| {
            client.handle();
            Ok(())
            // client.start()
            //     .and_then(move |response| {
            //         println!("CLIENT: {:?}", response);
            //         client.call("Goodbye".to_string())
            //     })
            // .and_then(|response| {
            //     println!("CLIENT: {:?}", response);
            //     Ok(())
            // })
            // We provide a way to *instantiate* the service for each new
            // connection; here, we just immediately return a new instance.
        });

    core.run(tcp).unwrap();
}
