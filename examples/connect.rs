extern crate xmpp_client;
#[macro_use]extern crate xmpp_derive;
extern crate tokio_core;
extern crate futures;
extern crate env_logger;
extern crate log;

use std::io;
use std::str::FromStr;
use tokio_core::reactor::Core;
use futures::Future;
use futures::future;
use futures::Stream;
use xmpp_client::ToJid;
use xmpp_client::{XMPPConfig, Credentials, Jid};
use tokio_core::net::TcpStream;
use xmpp_client::events::GenericMessage;
use xmpp_client::events::Event::Stanza;
use xmpp_client::events::Event::NonStanza;
use xmpp_client::events::StanzaEvent;
use xmpp_client::events::NonStanzaEvent;
use xmpp_client::events::Message;
use xmpp_client::events::CloseStream;
use xmpp_client::events::Event;
use xmpp_client::events::StanzaEvent::{MessageEvent};
use xmpp_client::events::EventTrait;
use futures::Async;

#[derive(Debug, Clone, XmppEvent)]
#[stanza(is="message")]
struct MessageOk {
    generic: GenericMessage
}

impl ToString for MessageOk {
    fn to_string(&self) -> String {
        format!("<message to='user1@example.com' from='alice@example.com' type='chat' id='purple6d50c1ba'><body>hey</body></message>")
    }
}

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
            handle.spawn(client.send_presence().then(move|_| {
                Ok(())
            }));

            let mut c = client.clone();
            let l = client.handle();
            l.and_then(move |stream| {
                stream.for_each(move |m| {
                    match m {
                        NonStanza(non_stanza, _) => match *non_stanza {
                            CloseStreamEvent => {
                                return Err(io::Error::new(io::ErrorKind::InvalidInput, ""));
                            }
                        },
                        Stanza(stanza, _) => match *stanza {
                            MessageEvent(_) => {
                                println!("New message");
                                client.send(CloseStream::new().to_event());
                            },
                            _ => {}
                        },
                        _ => {}
                    }
                    Ok(())
                })
            })
        })).unwrap();
}
