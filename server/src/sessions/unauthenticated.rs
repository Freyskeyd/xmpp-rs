use crate::{
    parser::codec::XmppCodec,
    sessions::{manager::SessionManager, state::SessionState, GetPacket, SessionManagementPacket, SessionManagementPacketResult},
};
use actix::{Actor, Context, Handler, SystemService};
use actix_codec::Encoder;
use bytes::BytesMut;
use log::{error, trace};
use tokio::{
    io::AsyncWriteExt,
    sync::mpsc::{Receiver, Sender},
};
use xmpp_proto::Packet;

#[derive(Default)]
pub(crate) struct UnauthenticatedSession {
    pub(crate) state: SessionState,
    pub(crate) packets: Vec<SessionManagementPacketResult>,
}

impl UnauthenticatedSession {
    pub(crate) async fn proceed_packet<W: AsyncWriteExt + Unpin>(
        packet: Packet,
        mut state: SessionState,
        tx: Sender<SessionManagementPacketResult>,
        rx: &mut Receiver<SessionManagementPacketResult>,
        codec: &mut XmppCodec,
        stream: &mut W,
        buf: &mut BytesMut,
    ) -> Result<SessionState, ()> {
        trace!("Sending packet to manager");
        let result = SessionManager::from_registry()
            .send(SessionManagementPacket {
                session_state: state,
                packet,
                referer: tx,
            })
            .await
            .unwrap();

        if result.is_ok() {
            trace!("Waiting for response");

            if let Some(SessionManagementPacketResult { session_state, packets }) = rx.recv().await {
                state = session_state;

                trace!("SessionState is {:?}", session_state);

                packets.into_iter().for_each(|packet| {
                    if let Err(e) = codec.encode(packet, buf) {
                        error!("Error: {:?}", e);
                    }
                });

                if let Err(e) = stream.write_buf(buf).await {
                    error!("{:?}", e);
                }

                if let Err(e) = stream.flush().await {
                    error!("{:?}", e);
                }
            }
            Ok(state)
        } else {
            Ok(state)
        }
    }
}

impl Actor for UnauthenticatedSession {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        trace!("Starting UnauthenticatedSession");
    }

    fn stopping(&mut self, _ctx: &mut Self::Context) -> actix::Running {
        trace!("Stopping UnauthenticatedSession");
        actix::Running::Stop
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        trace!("UnauthenticatedSession Stopped");
    }
}

impl Handler<SessionManagementPacketResult> for UnauthenticatedSession {
    type Result = ();

    fn handle(&mut self, packet: SessionManagementPacketResult, _ctx: &mut Self::Context) -> Self::Result {
        self.packets.push(packet);
    }
}
