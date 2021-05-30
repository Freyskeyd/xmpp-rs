use crate::sessions::{manager::SessionManager, state::SessionState, SessionManagementPacket, SessionManagementPacketResult};
use tokio::sync::mpsc::{self, Receiver, Sender};
use uuid::Uuid;
use xmpp_proto::OpenStreamBuilder;
use xmpp_proto::{NonStanza, Packet};

// The 'to' attribute SHOULD be used only in the XML stream header from the initiating entity to the receiving
// entity, and MUST be set to a hostname serviced by the receiving entity.

// There SHOULD NOT be a 'to' attribute set in the XML stream header by which the receiving entity replies to the initiating entity; however, if a 'to' attribute is included, it SHOULD be silently ignored by the initiating entity.

#[tokio::test]
async fn should_return_an_open_stream() {
    let handler = SessionManager::default();

    let (referer, mut rx): (Sender<SessionManagementPacketResult>, Receiver<SessionManagementPacketResult>) = mpsc::channel(32);
    let response = handler.handle_packet(SessionManagementPacket {
        session_state: SessionState::Opening,
        packet: OpenStreamBuilder::default().lang("en").version("1.0").id(Uuid::new_v4()).build().unwrap().into(),
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
