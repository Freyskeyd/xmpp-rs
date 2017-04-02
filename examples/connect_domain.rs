extern crate xmpp_client;
extern crate xmpp_proto;
extern crate tokio_core;
extern crate futures;
extern crate env_logger;
extern crate log;
extern crate unicode_width;
extern crate cursive;

use std::net::ToSocketAddrs;
use tokio_core::reactor::Core;
use futures::Future;
use futures::Stream;
use tokio_core::net::TcpStream;
use std::net::TcpStream as TcpStreamBase;

fn main() {
    env_logger::init().unwrap();

    let mut core = Core::new().unwrap();

    let handle = core.handle();
    // let addr = "127.0.0.1:5222".parse().unwrap();
    let x = "xmpp-qa.iadvize.com:5222".to_socket_addrs();
    // let tcp = TcpStreamBase::connect("xmpp-qa.iadvize.com:5222").unwrap();
    // let socket = TcpStream::from_stream(tcp, &handle).unwrap();

    // let runner = TcpStream::connect(&addr, &handle)
    let runner = 
            xmpp_client::Client::connect(socket)
            // .and_then(|mut client| {
            // client.send_presence()
            //     .and_then(move |_| {
            //         client.handle()
            //             .and_then(|stream| {
            //                 let ful = stream.for_each(|message| {
            //                     // Deal with Incomming Message
            //                     println!("Message: {:?}", message);
            //                     Ok(())
            //                 });

            //                 ful
            //             })
            //     })
        // })
    ;
    core.run(runner).unwrap();

}
