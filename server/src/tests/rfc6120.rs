use std::{error::Error, io, path::PathBuf, str::FromStr};

use crate::{
    parser::codec::XmppCodec,
    sessions::{manager::SessionManager, state::SessionState, SessionManagementPacket, SessionManagementPacketResult},
    Server,
};
use actix::{Addr, SystemService};
use actix_codec::Decoder;
use bytes::BytesMut;
use jid::Jid;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    sync::mpsc::{self, Receiver, Sender},
};
use uuid::Uuid;
use xmpp_proto::{NonStanza, Packet, StreamError, StreamErrorKind};
use xmpp_proto::{OpenStream, OpenStreamBuilder};

async fn executor(packet: impl Into<Packet>, expected_session_state: SessionState, resolver: impl Fn(Vec<Packet>) -> ()) -> Result<(), ()> {
    let sm = SessionManager::from_registry();
    let (referer, mut rx): (Sender<SessionManagementPacketResult>, Receiver<SessionManagementPacketResult>) = mpsc::channel(32);
    let _ = sm
        .send(SessionManagementPacket {
            session_state: SessionState::Opening,
            packet: packet.into(),
            referer,
        })
        .await
        .unwrap();

    if let Some(result) = rx.recv().await {
        assert_eq!(result.session_state, expected_session_state);
        resolver(result.packets);
        Ok(())
    } else {
        Err(())
    }
}

use demonstrate::demonstrate;

demonstrate! {
    describe "Opening a Stream" {
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
        async it "should accept a valid host when opening stream" -> Result<(), ()> {
            executor(packet, SessionState::Opening, |packets| {
                assert!(
                    matches!(
                        packets.as_slice(),
                        [Packet::NonStanza(open_stream), Packet::NonStanza(features)] if matches!(**open_stream, NonStanza::OpenStream(_))
                        && matches!(**features, NonStanza::StreamFeatures(_))
                    )
                );
            }).await
        }

        #[actix::test]
        async it "should fail on invalid host" -> Result<(), ()> {
            packet.to = Jid::from_str("invalid").ok();

            executor(packet, SessionState::Closing, |packets| {
                assert!(
                    matches!(packets.as_slice(), [Packet::NonStanza(open_stream), Packet::NonStanza(error), Packet::NonStanza(close)] if matches!(**open_stream, NonStanza::OpenStream(_))
                    && matches!(**error, NonStanza::StreamError(StreamError { kind: StreamErrorKind::HostUnknown }))
                    && matches!(**close, NonStanza::CloseStream(_))
                    )
                );
            }).await
        }
    }
}
