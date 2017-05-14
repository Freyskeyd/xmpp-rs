extern crate xmpp_client;
#[macro_use]
extern crate xmpp_derive;
extern crate tokio_core;
extern crate futures;
extern crate env_logger;
extern crate log;
extern crate elementtree;

use std::io;
use tokio_core::reactor::Core;
use futures::Future;
use futures::Stream;
use xmpp_client::ToJid;
use xmpp_client::{XMPPConfig, Credentials, Jid};
use tokio_core::net::TcpStream;
use xmpp_client::events::GenericMessage;
use xmpp_client::events::Event::Stanza;
use xmpp_client::events::Event::NonStanza;
use xmpp_client::events::StanzaEvent;
use xmpp_client::events::NonStanzaEvent;
use xmpp_client::events::Event;
use xmpp_client::events::StanzaEvent::MessageEvent;
use xmpp_client::events::ToEvent;
use xmpp_client::events::ToXmlElement;
use xmpp_client::events::FromXmlElement;
use xmpp_client::Element;

#[derive(Debug, Clone, XmppEvent)]
#[stanza(is="message")]
struct MessageOk {
    generic: GenericMessage,
}

impl ToXmlElement for MessageOk {
    type Error = io::Error;
    fn to_element(&self) -> Result<Element, Self::Error> {
        println!("MessageOk:: to_element called");
        let mut element = self.generic.to_element().unwrap();

        element.append_new_child(("", "body")).set_text("hey");

        Ok(element)
    }
}

fn main() {
    env_logger::init().unwrap();

    let mut core = Core::new().unwrap();

    let handle = core.handle();
    let addr = "127.0.0.1:5222".parse().unwrap();

    let config = XMPPConfig::new().set_domain("example.com");

    let credentials = Credentials {
        jid: Jid::from_full_jid("alice@example.com"),
        password: String::from("test"),
    };
    core.run(TcpStream::connect(&addr, &handle)
                 .and_then(|stream| xmpp_client::Client::connect(stream, config, Some(credentials)))
                 .and_then(|mut client| {
            handle.spawn(client.send_presence().then(move |_| Ok(())));

            let l = client.handle();
            l.and_then(move |stream| {
                stream.for_each(move |m| {
                    match m {
                        NonStanza(non_stanza) => {
                            match *non_stanza {
                                NonStanzaEvent::CloseStreamEvent => {
                                    return Err(io::Error::new(io::ErrorKind::InvalidInput, ""));
                                }
                                _ => {}
                            }
                        }
                        Stanza(stanza) => {
                            match *stanza {
                                MessageEvent(_) => {
                                    println!("New message");
                                    let e = MessageOk { generic: GenericMessage::new("user1@example.com/MacBook-Pro-de-Simon".to_jid().unwrap()) };

                                    let x = e.to_event();
                                    println!("");
                                    println!("Compiled event: {:?}", x);
                                    println!("");
                                    client.send(x);
                                }
                                _ => {}
                            }
                        }
                    }
                    Ok(())
                })
            })
        }))
        .unwrap();
}
