use crate::sessions::{manager::SessionManager, state::SessionState, SessionManagementPacket, SessionManagementPacketResult};
use actix::SystemService;
use tokio::sync::mpsc::{self, Receiver, Sender};
use xmpp_proto::Packet;

pub(crate) async fn executor(packet: impl Into<Packet>, expected_session_state: SessionState, starting_state: SessionState, resolver: impl Fn(Vec<Packet>) -> ()) -> Result<(), ()> {
    let sm = SessionManager::from_registry();
    let (referer, mut rx): (Sender<SessionManagementPacketResult>, Receiver<SessionManagementPacketResult>) = mpsc::channel(32);
    let _ = sm
        .send(SessionManagementPacket {
            session_state: starting_state,
            packet: packet.into(),
            referer,
        })
        .await
        .unwrap();

    if let Some(result) = rx.recv().await {
        assert_eq!(
            result.session_state, expected_session_state,
            "SessionState wasn't matching: expected: {:?} | received: {:?}",
            expected_session_state, result.session_state
        );
        resolver(result.packets);
        Ok(())
    } else {
        println!("executor fail");
        Err(())
    }
}

#[macro_export]
macro_rules! execute {
    ($packet:ident, starting_state $starting_state: expr, expected_state $state:expr, $( $pattern:pat )|+ $( if $guard: expr )? $(,)?) => {
        executor($packet, $state, $starting_state, |packets| {
            assert!(matches!(packets.as_slice(), $( $pattern )|+ $( if $guard )?), "Packets didn't matches expected ones: {:#?}", packets);
        })
        .await
    };

    ($packet:ident, $state:expr, $( $pattern:pat )|+ $( if $guard: expr )? $(,)?) => {
        execute!($packet, starting_state SessionState::Opening, expected_state $state, $( $pattern )|+ $( if $guard )?)
    };

}
