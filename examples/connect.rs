extern crate xmpp_client;
extern crate tokio_core;
extern crate futures;
extern crate env_logger;
extern crate log;

use tokio_core::reactor::Core;
use futures::Future;
use futures::Stream;
use xmpp_client::{XMPPConfig, Credentials, Jid};
use tokio_core::net::TcpStream;
use xmpp_client::events::Event::Stanza;
use xmpp_client::events::StanzaEvent::{MessageEvent};

fn main() {
    env_logger::init().unwrap();

    let mut core = Core::new().unwrap();

    let handle = core.handle();
    let addr = "127.0.0.1:5222".parse().unwrap();

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
            let ping = client.send_ping()
                .then(move |x| {
                    println!("X: {:?}", x);
                    Ok(())
                });

            handle.spawn(ping);
            handle.spawn(client.send_presence().then(move|_| {
                Ok(())
            }));

            client.handle().and_then(move |stream| {
                stream.for_each(move |m| {
                    match m {
                        Stanza(MessageEvent(_), _) => {
                            println!("New message");
                        }
                        _ => {}
                    }
                    Ok(())
                })
            })
        })).unwrap();
}
