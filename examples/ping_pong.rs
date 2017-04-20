extern crate xmpp_client;
#[macro_use]extern crate xmpp_derive;
extern crate tokio_core;
extern crate futures;
extern crate env_logger;
extern crate log;

use std::str::FromStr;
use tokio_core::reactor::Core;
use futures::Future;
use futures::Stream;
use xmpp_client::{XMPPConfig, Credentials, Jid};
use tokio_core::net::TcpStream;
use xmpp_client::events::Event::Stanza;
use xmpp_client::events::StanzaEvent;
use xmpp_client::events::IqType;
use xmpp_client::events::Message;
use xmpp_client::events::CloseStream;
use xmpp_client::events::Ping;
use xmpp_client::events::Event;
use xmpp_client::events::IqEvent;
use xmpp_client::events::StanzaEvent::{IqRequestEvent, MessageEvent};
use xmpp_client::events::EventTrait;
use std::thread;

fn main() {
    env_logger::init().unwrap();

    thread::spawn(|| {

        let mut core = Core::new().unwrap();

        let handle = core.handle();
        let addr = "127.0.0.1:5222".parse().unwrap();

        let config = XMPPConfig::new()
            .set_domain("example.com");

        let credentials = Credentials {
            jid: Jid::from_full_jid("alice@example.com"),
            password: String::from("test")
        };
        thread::sleep_ms(1000);
        core.run(
            TcpStream::connect(&addr, &handle).and_then(|stream| {
                xmpp_client::Client::connect(stream, config, Some(credentials))
            }).and_then(|mut client| {
                handle.spawn(client.send_presence().then(move|_| {
                    Ok(())
                }));

                client.handle().and_then(move |stream| {
                    stream.for_each(move |m| {
                        Ok(match m {
                            Stanza(ref stanza, _) => match **stanza {
                                IqRequestEvent(ref iq) => {
                                    match **iq {
                                        IqEvent::PingEvent(ref ping) => {
                                            let mut p = Ping::new("", ping.get_from().unwrap());
                                            let _ = p.set_id(ping.get_id());
                                            let _ = p.set_type(IqType::Result);

                                            client.send(p.to_event());
                                            handle.spawn(client.send(CloseStream::new().to_event()).then(move |_| {Ok(())}));
                                            ()
                                        },
                                        _ => {}
                                    }
                                }
                                _ => ()
                            },
                            _ => ()
                        })
                    })
                })
            })).unwrap();
    });

    let mut core = Core::new().unwrap();

    let handle = core.handle();
    let addr = "127.0.0.1:5222".parse().unwrap();

    let config = XMPPConfig::new()
        .set_domain("example.com");

    let credentials = Credentials {
        jid: Jid::from_full_jid("user1@example.com"),
        password: String::from("test")
    };
    core.run(
        TcpStream::connect(&addr, &handle).and_then(|stream| {
            xmpp_client::Client::connect(stream, config, Some(credentials))
        }).and_then(|mut client| {
            handle.spawn(client.send_presence().then(move|_| {
                Ok(())
            }));

//             let c = client.clone();
            client.handle().and_then(move |stream| {
                stream.for_each(move |m| {
                    Ok(match m {
                        Stanza(stanza, _) => match *stanza {
                            StanzaEvent::PresenceEvent(p) => {
                                match p.get_from() {
                                    Some(jid) => match jid.node {
                                        Some(ref node) => {
                                            if node == "alice" {
                                                let mut p = Ping::new("", jid);
                                                let mut c = client.clone();
                                                let ping = client.send_ping(&mut p)
                                                    .then(move |x| {
                                                        println!("X: {:?}", x);
                                                        c.send(CloseStream::new().to_event());
                                                        // c.shutdown();
                                                        Ok(())
                                                    });

                                                handle.spawn(ping)
                                            }
                                        },
                                        _ => {}
                                    },
                                    None => {}
                                }
                            },
                            _ => {}
                        },
                        _ => {}
                    })
                    // Ok(())
                })
            })
        })).unwrap();
}
