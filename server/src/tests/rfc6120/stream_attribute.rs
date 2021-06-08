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

        describe "the to attribute" {
            use super::*;
            use xmpp_proto::OpenStream;

            #[actix::test]
            async it "must be defined" -> Result<(), ()> {
                packet.to = None;
                execute!(packet, SessionState::Closing,
                    [Packet::NonStanza(open_stream), Packet::NonStanza(error), Packet::NonStanza(close)]
                    if matches!(**open_stream, NonStanza::OpenStream(_)) &&
                    matches!(**error, NonStanza::StreamError(StreamError { kind: StreamErrorKind::HostUnknown, .. })) &&
                    matches!(**close, NonStanza::CloseStream(_))
                )
            }

            #[actix::test]
            async it "must be set to the bare jid of the initial stream header" -> Result<(), ()> {
                packet.from = Jid::from_str("alice@wonderland.lit/rabbithole").ok();
                let expected_jid = Jid::from_str("alice@wonderland.lit").ok();

                execute!(packet, SessionState::Opening,
                    [Packet::NonStanza(open_stream), ..]
                    if matches!(**open_stream, NonStanza::OpenStream(OpenStream {ref to, ..}) if to == &expected_jid)
                )
            }

        }

        describe "the id attribute" {
            use super::*;
            use xmpp_proto::OpenStream;

            #[actix::test]
            async it "must be ignored if defined by initiating entity" -> Result<(), ()> {
                let expected_id = packet.id.clone();

                execute!(packet, SessionState::Opening,
                    [Packet::NonStanza(open_stream), ..]
                    if matches!(**open_stream, NonStanza::OpenStream(OpenStream { ref id, .. }) if id != &expected_id)
                )
            }
        }

        describe "the lang attribute" {
            use super::*;
            use xmpp_proto::OpenStream;

            #[actix::test]
            async it "should be defined by initiating entity" -> Result<(), ()> {

                execute!(packet, SessionState::Opening,
                    [Packet::NonStanza(open_stream), ..]
                    if matches!(**open_stream, NonStanza::OpenStream(OpenStream { ref lang, .. }) if lang == "en")
                )
            }

            #[actix::test]
            async it "should be the default server lang if the initiating entity submit an unsupported lang" -> Result<(), ()> {
                packet.lang = "it".to_string();

                execute!(packet, SessionState::Opening,
                    [Packet::NonStanza(open_stream), ..]
                    if matches!(**open_stream, NonStanza::OpenStream(OpenStream { ref lang, .. }) if lang == "en")
                )
            }
        }

        describe "the version attribute" {
            use super::*;
            use xmpp_proto::OpenStream;

            #[actix::test]
            async it "should be 1_0" -> Result<(), ()> {

                execute!(packet, SessionState::Opening,
                    [Packet::NonStanza(open_stream), ..]
                    if matches!(**open_stream, NonStanza::OpenStream(OpenStream { ref version, .. }) if version == "1.0")
                )
            }

            #[actix::test]
            async it "if defined under 1_0 generate an error" -> Result<(), ()> {
                packet.version = "0.9".into();

                execute!(packet, SessionState::Closing,
                    [Packet::NonStanza(open_stream), Packet::NonStanza(error), Packet::NonStanza(close)]
                    if matches!(**open_stream, NonStanza::OpenStream(_)) &&
                    matches!(**error, NonStanza::StreamError(StreamError { kind: StreamErrorKind::UnsupportedVersion, .. })) &&
                    matches!(**close, NonStanza::CloseStream(_))
                )
            }
        }
    }
}
