use std::{error::Error, io, path::PathBuf};

use crate::{
    parser::codec::XmppCodec,
    sessions::{manager::SessionManager, state::SessionState, SessionManagementPacket, SessionManagementPacketResult},
    Server,
};
use actix::SystemService;
use actix_codec::Decoder;
use bytes::BytesMut;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    sync::mpsc::{self, Receiver, Sender},
};
use uuid::Uuid;
use xmpp_proto::{NonStanza, Packet, StreamError, StreamErrorKind};
use xmpp_proto::{OpenStream, OpenStreamBuilder};

#[tokio::test]
async fn should_return_an_open_stream() {
    let handler = SessionManager::default();

    let (referer, mut rx): (Sender<SessionManagementPacketResult>, Receiver<SessionManagementPacketResult>) = mpsc::channel(32);
    let response = handler.handle_packet(SessionManagementPacket {
        session_state: SessionState::Opening,
        packet: OpenStreamBuilder::default().to("localhost").lang("en").version("1.0").id(Uuid::new_v4()).build().unwrap().into(),
        referer,
    });

    assert!(response.is_ok());

    if let Some(result) = rx.recv().await {
        assert_eq!(result.session_state, SessionState::Opening);
        assert!(
            matches!(result.packets.as_slice(), [Packet::NonStanza(open_stream), Packet::NonStanza(features)] if matches!(**open_stream, NonStanza::OpenStream(_))
                && matches!(**features, NonStanza::StreamFeatures(_)))
        );
    } else {
        panic!("Should have respond something");
    }
}

#[actix::test]
async fn should_return_an_open_stream_2() -> Result<(), Box<dyn Error>> {
    let (referer, mut rx): (Sender<SessionManagementPacketResult>, Receiver<SessionManagementPacketResult>) = mpsc::channel(32);

    let response = SessionManager::from_registry()
        .send(SessionManagementPacket {
            session_state: SessionState::Opening,
            packet: OpenStreamBuilder::default().to("unknownhost").lang("en").version("1.0").id(Uuid::new_v4()).build().unwrap().into(),
            referer,
        })
        .await;

    assert!(response.is_ok());

    if let Some(result) = rx.recv().await {
        assert_eq!(result.session_state, SessionState::Closing);
        assert!(
            matches!(result.packets.as_slice(), [Packet::NonStanza(open_stream), Packet::NonStanza(error), Packet::NonStanza(close)] if matches!(**open_stream, NonStanza::OpenStream(_))
            && matches!(**error, NonStanza::StreamError(StreamError { kind: StreamErrorKind::HostUnknown }))
            && matches!(**close, NonStanza::CloseStream(_))
            )
        );
    } else {
        panic!("Should have respond something");
    }

    Ok(())
}
