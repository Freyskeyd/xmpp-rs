use crate::{execute, sessions::state::SessionState, tests::executor::executor};
use demonstrate::demonstrate;
use xmpp_proto::{NonStanza, Packet, StreamError, StreamErrorKind};

demonstrate! {
    describe "when opening a Stream" {
        use super::*;

        #[actix::test]
        async it "should fail on invalid namespace" -> Result<(), ()> {
            let packet = Packet::InvalidPacket(Box::new(StreamErrorKind::InvalidNamespace));

            execute!(packet, SessionState::Closing,
                [Packet::NonStanza(open_stream), Packet::NonStanza(error), Packet::NonStanza(close)]
                if matches!(**open_stream, NonStanza::OpenStream(_)) &&
                   matches!(**error, NonStanza::StreamError(StreamError { kind: StreamErrorKind::InvalidNamespace })) &&
                   matches!(**close, NonStanza::CloseStream(_))
            )
        }

        #[actix::test]
        async it "should fail on bad prefix namespace" -> Result<(), ()> {
            let packet = Packet::InvalidPacket(Box::new(StreamErrorKind::BadNamespacePrefix));

            execute!(packet, SessionState::Closing,
                [Packet::NonStanza(open_stream), Packet::NonStanza(error), Packet::NonStanza(close)]
                if matches!(**open_stream, NonStanza::OpenStream(_)) &&
                   matches!(**error, NonStanza::StreamError(StreamError { kind: StreamErrorKind::BadNamespacePrefix })) &&
                   matches!(**close, NonStanza::CloseStream(_))
            )
        }


    }
}
