extern crate xmpp as line;
extern crate futures;
extern crate tokio_core;
extern crate tokio_proto;
extern crate tokio_service;
extern crate byteorder;
#[macro_use]
extern crate log;
extern crate env_logger;

use std::env;
use tokio_core::reactor::Core;
use futures::{Future};
use tokio_service::{Service};
use std::{str};
use std::net::SocketAddr;
use line::XmppService;

fn main() {
    env_logger::init().unwrap();
    // The builder requires a protocol and an address
    // let server = TcpServer::new(LineProto, addr);

    let addr = env::args().nth(1).unwrap_or("127.0.0.1:5222".to_string());
    let addr = addr.parse::<SocketAddr>().unwrap();

    // Create the event loop and initiate the connection to the remote server
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let tcp = line::XmppClient::connect(&addr, &handle)
        .and_then(|client| {
            client.start()
                .and_then(move|response| {
                    loop {
                        match client.handle() {
                            line::Event::StreamOpened => println!("Stream opened!")
                        }
                    }
                    Ok(())
                })
        });

    core.run(tcp).unwrap();
}
