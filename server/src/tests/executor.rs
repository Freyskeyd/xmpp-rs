use crate::packet::PacketHandler;
use crate::sessions::state::StaticSessionState;
use crate::sessions::{state::SessionState, unauthenticated::UnauthenticatedSession};
use xmpp_proto::Packet;

pub(crate) async fn executor(packet: impl Into<Packet>, expected_session_state: SessionState, starting_state: SessionState, resolver: impl Fn(Vec<Packet>) -> ()) -> Result<(), ()> {
    let s = StaticSessionState::builder().state(starting_state).build().unwrap();
    let res = UnauthenticatedSession::handle_packet(s, &packet.into()).await;
    if let Ok(result) = res {
        assert_eq!(
            result.session_state.state, expected_session_state,
            "SessionState wasn't matching: expected: {:?} | received: {:?}",
            expected_session_state, result.session_state.state
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
