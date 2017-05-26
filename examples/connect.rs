extern crate xmpp_client;
extern crate xmpp_config;
extern crate xmpp_credentials;
extern crate xmpp_events;
extern crate xmpp_jid;
#[macro_use]
extern crate xmpp_derive;
extern crate tokio_core;
extern crate futures;
extern crate env_logger;
extern crate log;

use std::io;
use tokio_core::reactor::Core;
use futures::Future;
use futures::Stream;
use xmpp_config::XMPPConfig;
use xmpp_credentials::Credentials;
use xmpp_jid::ToJid;
use xmpp_jid::Jid;
use tokio_core::net::TcpStream;
use xmpp_events::GenericMessage;
use xmpp_events::Event::Stanza;
use xmpp_events::Event::NonStanza;
use xmpp_events::StanzaEvent;
use xmpp_events::NonStanzaEvent;
use xmpp_events::Event;
use xmpp_events::StanzaEvent::MessageEvent;
use xmpp_events::ToEvent;
use xmpp_events::ToXmlElement;
use xmpp_events::FromXmlElement;
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
