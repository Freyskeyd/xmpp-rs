use crate::sessions::{state::SessionState, SessionManagementPacketResultBuilder};
use actix::{Actor, Context, Handler, Message, Recipient, Supervised, SystemService};

use log::trace;
use tokio::sync::mpsc::Sender;

use xmpp_proto::{CloseStream, Features, OpenStream, StreamError, StreamErrorKind};

use super::SessionManagementPacketResult;

/// Manage sessions on a node
#[derive(Default)]
pub struct SessionManager {
    // sessions: HashMap<String, HashMap<String, Session>>,
}

impl SessionManager {
    pub(crate) fn new() -> Self {
        // Self { sessions: HashMap::new() }
        Self {}
    }

    pub(crate) fn not_authorized_and_close(response: &mut SessionManagementPacketResultBuilder) -> Result<SessionManagementPacketResult, ()> {
        response
            .packet(StreamError { kind: StreamErrorKind::NotAuthorized }.into())
            .packet(CloseStream {}.into())
            .session_state(SessionState::Closing)
            .build()
            .map_err(|_| ())
    }

    pub(crate) fn handle_invalid_packet(
        session_state: &SessionState,
        invalid_packet: &StreamErrorKind,
        response: &mut SessionManagementPacketResultBuilder,
    ) -> Result<SessionManagementPacketResult, ()> {
        if matches!(*invalid_packet, StreamErrorKind::UnsupportedEncoding) && SessionState::Opening.eq(session_state) {
            return response.session_state(SessionState::UnsupportedEncoding).build().map_err(|_| ());
        }

        match session_state {
            SessionState::Opening => {
                response.packet(OpenStream::default().into());
            }
            _ => {}
        }

        response
            .packet(StreamError { kind: invalid_packet.clone() }.into())
            .packet(CloseStream {}.into())
            .session_state(SessionState::Closing)
            .build()
            .map_err(|_| ())
    }
}

impl Supervised for SessionManager {}
impl SystemService for SessionManager {}
impl Actor for SessionManager {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        trace!("SessionManager started");
    }
}

#[derive(Debug, Message)]
#[rtype("Result<(),()>")]
pub(crate) struct RegistrationStatus {}

#[derive(Debug, Message)]
#[rtype("Result<(),()>")]
pub(crate) struct RegisterSession {
    pub(crate) referer: Recipient<RegistrationStatus>,
}
impl Handler<RegisterSession> for SessionManager {
    type Result = Result<(), ()>;

    fn handle(&mut self, msg: RegisterSession, _ctx: &mut Self::Context) -> Self::Result {
        println!("Registering session");

        let _ = msg.referer.do_send(RegistrationStatus {});
        Ok(())
    }
}

#[derive(Debug, Message)]
#[rtype("Result<Features,()>")]
pub(crate) struct GetMechanisms(pub(crate) String);
impl Handler<GetMechanisms> for SessionManager {
    type Result = Result<Features, ()>;

    fn handle(&mut self, _: GetMechanisms, _ctx: &mut Self::Context) -> Self::Result {
        Ok(Features::Mechanisms(vec!["PLAIN".into()]))
    }
}
