use crate::{execute, sessions::state::SessionState, tests::executor::executor};
use demonstrate::demonstrate;
use jid::Jid;
use std::str::FromStr;
use uuid::Uuid;
use xmpp_proto::OpenStreamBuilder;
use xmpp_proto::{NonStanza, Packet, StreamError, StreamErrorKind};

demonstrate! {
    describe "when opening a Stream" {
        use super::*;


        before {
            let host: Jid = Jid::from_str("localhost").unwrap();
            let lang: String = "en".into();
            let version: String = "1.0".into();

            let mut packet = OpenStreamBuilder::default()
                .to(host)
                .lang(lang)
                .version(version)
                .id(Uuid::new_v4().to_string())
                .build()
                .unwrap();
        }

        describe "the from attribute" {
            use super::*;

            #[actix::test]
            async it "can be missing" -> Result<(), ()> {
                execute!(packet, SessionState::Opening, [Packet::NonStanza(open_stream), ..] if matches!(**open_stream, NonStanza::OpenStream(_)))
            }

            #[actix::test]
            async it "can be defined" -> Result<(), ()> {
                packet.from = Jid::from_str("alice@wonderland.lit").ok();

            execute!(packet, SessionState::Opening, [Packet::NonStanza(open_stream), ..] if matches!(**open_stream, NonStanza::OpenStream(_)))
            }
        }
    }
}
