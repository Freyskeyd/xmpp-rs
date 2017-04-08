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
use xmpp_proto::events::{Message};
use xmpp_proto::events::Event::Stanza;
use xmpp_proto::events::StanzaEvent::MessageEvent;
use xmpp_proto::credentials::Credentials;
use xmpp_proto::jid::Jid;
use std::{thread, time};

use tokio_core::reactor::Handle;
use xmpp_client::Client;
use futures::future;

fn main() {
    env_logger::init().unwrap();

    let mut core = Core::new().unwrap();

    let handle = core.handle();
    let addr = "127.0.0.1:5222".parse().unwrap();
    // let addrs: Vec<SocketAddr> = "xmpp-qa.iadvize.com:5222".to_socket_addrs().unwrap().collect();
    // let addr = addrs[0];

    let config = XMPPConfig::new()
          .set_domain("example.com");

    let credentials = Credentials {
        jid: Jid::from_full_jid("alice@example.com"),
        password: String::from("test")
    };
    core.run(
        TcpStream::connect(&addr, &handle).and_then(|stream| {
            xmpp_client::Client::connect(stream, config, Some(credentials))
        }).and_then(|mut client| {
            handle.spawn(client.ping()
                         .then(move|x| {
                             println!("X: {:?}", x);
                             Ok(())
                         }));

            handle.spawn(client.send_presence().then(move|_| {
                Ok(())
            }));

            client.handle().and_then(move |stream| {

                stream.for_each(move |m| {
                    match m {
                        Stanza(MessageEvent(_), _) => {
                            println!("New message");
                            let jid = client.get_jid();
                            let event = Stanza(MessageEvent(Message::new(&jid.jid.to_string(), "user1@example.com/MacBook-Pro-de-Simon")), String::new());
                            handle.spawn(client.send(event).then(move|_| {Ok(())}));
                        }
                        _ => {}
                    }
                    Ok(())
                })
            })
        })).unwrap();

}
