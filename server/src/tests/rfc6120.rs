use crate::{execute, sessions::state::SessionState, tests::executor::executor};
use demonstrate::demonstrate;
use jid::Jid;
use std::str::FromStr;
use uuid::Uuid;
use xmpp_proto::OpenStreamBuilder;
use xmpp_proto::{NonStanza, Packet, StreamError, StreamErrorKind};

mod namespaces;
mod starttls;
mod stream_attribute;

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

        #[actix::test]
        async it "should accept a valid host" -> Result<(), ()> {
            execute!(packet, SessionState::Opening, [Packet::NonStanza(open_stream), ..] if matches!(**open_stream, NonStanza::OpenStream(_)))
        }

        #[actix::test]
        async it "should fail on invalid host" -> Result<(), ()> {
            packet.to = Jid::from_str("invalid").ok();

            execute!(packet, SessionState::Closing,
                [Packet::NonStanza(open_stream), Packet::NonStanza(error), Packet::NonStanza(close)]
                if matches!(**open_stream, NonStanza::OpenStream(_)) &&
                   matches!(**error, NonStanza::StreamError(StreamError { kind: StreamErrorKind::HostUnknown, .. })) &&
                   matches!(**close, NonStanza::CloseStream(_))
            )
        }

        #[actix::test]
        async it "should fail on unsupported encoding" -> Result<(), ()> {
            let fail_packet = Packet::InvalidPacket(Box::new(StreamErrorKind::UnsupportedEncoding));

            assert!(execute!(fail_packet, SessionState::UnsupportedEncoding, []).is_ok());
            execute!(
                packet,
                starting_state SessionState::UnsupportedEncoding,
                expected_state SessionState::Closing,
                [Packet::NonStanza(open_stream), Packet::NonStanza(error), Packet::NonStanza(close)]
                if matches!(**open_stream, NonStanza::OpenStream(_)) &&
                   matches!(**error, NonStanza::StreamError(StreamError { kind: StreamErrorKind::UnsupportedEncoding, .. })) &&
                   matches!(**close, NonStanza::CloseStream(_))
            )
        }
    }
}
