use xmpp_proto::{CloseStream, OpenStream, Packet, StreamError, StreamErrorKind};

use crate::messages::SessionManagementPacketResult;
use crate::messages::SessionManagementPacketResultBuilder;
use crate::sessions::state::SessionState;
#[async_trait::async_trait]
pub(crate) trait PacketHandler {
    type Result;
    type From;

    async fn handle_packet(state: &SessionState, stanza: &Packet, from: Self::From) -> Self::Result;

    fn handle_invalid_packet(session_state: &SessionState, invalid_packet: &StreamErrorKind, response: &mut SessionManagementPacketResultBuilder) -> Result<SessionManagementPacketResult, ()> {
        if matches!(*invalid_packet, StreamErrorKind::UnsupportedEncoding) && SessionState::Opening.eq(session_state) {
            return response.session_state(SessionState::UnsupportedEncoding).build().map_err(|_| ());
        }

        if let SessionState::Opening = session_state {
            response.packet(OpenStream::default().into());
        }

        response
            .packet(StreamError { kind: invalid_packet.clone() }.into())
            .packet(CloseStream {}.into())
            .session_state(SessionState::Closing)
            .build()
            .map_err(|_| ())
    }

    fn not_authorized_and_close(response: &mut SessionManagementPacketResultBuilder) -> Result<SessionManagementPacketResult, ()> {
        response
            .packet(StreamError { kind: StreamErrorKind::NotAuthorized }.into())
            .packet(CloseStream {}.into())
            .session_state(SessionState::Closing)
            .build()
            .map_err(|_| ())
    }
}

#[async_trait::async_trait]
pub(crate) trait StanzaHandler<T> {
    async fn handle(state: &SessionState, stanza: &T) -> Result<SessionManagementPacketResult, ()>;
}
