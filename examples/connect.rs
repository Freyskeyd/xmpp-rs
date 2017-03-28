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

fn main() {
    env_logger::init().unwrap();

    let mut core = Core::new().unwrap();

    let handle = core.handle();
    let addr = "127.0.0.1:5222".parse().unwrap();
    core.run(
        TcpStream::connect(&addr, &handle).and_then(|stream| {
            xmpp_client::Client::connect(stream)
        }).and_then(|mut client| {
            // let writer = client.handle_write(rx);
            client.send_presence()
                .and_then(move |_| {
                    client.handle()
                        .and_then(|stream| {
                            let ful = stream.for_each(|message| {
                                // Deal with Incomming Message
                                println!("Message: {:?}", message);
                                Ok(())
                            });

                            ful
                        })
                })
        }
        )).unwrap();

}
