extern crate xmpp_client;
extern crate xmpp_proto;
extern crate tokio_core;
extern crate futures;
extern crate env_logger;
extern crate log;
extern crate unicode_width;
extern crate cursive;

use tokio_core::reactor::Core;
use futures::Future;
use futures::Stream;
use tokio_core::net::TcpStream;
use xmpp_proto::config::XMPPConfig;

fn main() {
    env_logger::init().unwrap();

    let mut core = Core::new().unwrap();

    let handle = core.handle();

    let addr = "127.0.0.1:5222".parse().unwrap();

    let config = XMPPConfig::new()
        .set_domain("example.com");

    core.run(
        TcpStream::connect(&addr, &handle).and_then(|stream| {
            xmpp_client::Client::connect(stream, config)
        }).and_then(|mut client| {
            let c = client.clone();
            client.send_presence()
                .and_then(move |_| {
                    client.handle()
                        .and_then(|stream| {
                            stream.for_each(move |message| {
                                // Deal with Incomming Message
                                if message.contains("body") {
                                    println!("Message: {:?}", message);
                                    let x = c.send_presence().map(|_| {
                                    // ... after which we'll print what happened
                                        println!("wrote bytes");
                                    }).map_err(|err| {
                                        println!("IO error {:?}", err)
                                    });
                                    handle.spawn(x);
                                } else {
                                    println!("No Body");
                                }
                                Ok(())
                            })
                        })
                })
        }
        )).unwrap();

}
