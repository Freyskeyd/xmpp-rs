use crate::messages::system::GetMechanisms;
use crate::{
    authentication::AuthenticationManager,
    packet::{PacketHandler, StanzaHandler},
    sessions::SessionManagementPacketResultBuilder,
    sessions::{manager::SessionManager, state::SessionState, SessionManagementPacketResult},
    AuthenticationRequest,
};
use actix::{Actor, Context, SystemService};
use jid::{BareJid, Jid};
use log::{error, trace};
use std::str::FromStr;
use tokio::sync::mpsc::Sender;
use uuid::Uuid;
use xmpp_proto::{Auth, Bind, CloseStream, Features, NonStanza, OpenStream, Packet, ProceedTls, SASLSuccess, Stanza, StartTls, StreamError, StreamErrorKind, StreamFeatures};

#[derive(Default)]
pub(crate) struct UnauthenticatedSession {
    pub(crate) state: SessionState,
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

#[async_trait::async_trait]
impl PacketHandler for UnauthenticatedSession {
    type Result = Result<(), ()>;
    type From = Option<Sender<SessionManagementPacketResult>>;

    async fn handle_packet(state: &SessionState, stanza: &Packet, from: Self::From) -> Self::Result {
        match stanza {
            Packet::NonStanza(stanza) => Self::handle(state, &**stanza).await,
            Packet::Stanza(stanza) => Self::handle(state, &**stanza).await,
            Packet::InvalidPacket(invalid_packet) => {
                let mut response = SessionManagementPacketResultBuilder::default();

                Self::handle_invalid_packet(state, invalid_packet, &mut response)
            }
        }
        .map(|result| result.send(from))
    }
}

#[async_trait::async_trait]
impl StanzaHandler<Stanza> for UnauthenticatedSession {
    async fn handle(_state: &SessionState, _stanza: &Stanza) -> Result<SessionManagementPacketResult, ()> {
        let fut = Box::pin(async { Err(()) });

        fut.await
    }
}

#[async_trait::async_trait]
impl StanzaHandler<NonStanza> for UnauthenticatedSession {
    async fn handle(state: &SessionState, stanza: &NonStanza) -> Result<SessionManagementPacketResult, ()> {
        let fut = match stanza {
            NonStanza::OpenStream(stanza) => Self::handle(state, stanza),
            NonStanza::StartTls(stanza) => Self::handle(state, stanza),
            NonStanza::Auth(stanza) => Self::handle(state, stanza),
            NonStanza::StreamError(stanza) => Self::handle(state, stanza),
            NonStanza::CloseStream(stanza) => Self::handle(state, stanza),
            _ => Box::pin(async { Err(()) }),
        };

        fut.await
    }
}

#[async_trait::async_trait]
impl StanzaHandler<CloseStream> for UnauthenticatedSession {
    async fn handle(_state: &SessionState, _stanza: &CloseStream) -> Result<SessionManagementPacketResult, ()> {
        let mut response = SessionManagementPacketResultBuilder::default();

        response.session_state(SessionState::Closing).packet(CloseStream {}.into()).build().map_err(|_| ())
    }
}

#[async_trait::async_trait]
impl StanzaHandler<OpenStream> for UnauthenticatedSession {
    async fn handle(state: &SessionState, stanza: &OpenStream) -> Result<SessionManagementPacketResult, ()> {
        let mut response = SessionManagementPacketResultBuilder::default();

        response.packet(
            OpenStream {
                id: Some(Uuid::new_v4().to_string()),
                to: stanza.from.as_ref().map(|jid| BareJid::from(jid.clone()).into()),
                // TODO: Replace JID crate with another?
                // TODO: Validate FQDN
                from: Jid::from_str("localhost").ok(),
                // TODO: Validate lang input
                lang: "en".into(),
                version: "1.0".to_string(),
            }
            .into(),
        );

        if SessionState::UnsupportedEncoding.eq(state) {
            return response
                .packet(
                    StreamError {
                        kind: StreamErrorKind::UnsupportedEncoding,
                    }
                    .into(),
                )
                .packet(CloseStream {}.into())
                .session_state(SessionState::Closing)
                .build()
                .map_err(|_| ());
        }

        if stanza.version != "1.0" {
            return response
                .packet(
                    StreamError {
                        kind: StreamErrorKind::UnsupportedVersion,
                    }
                    .into(),
                )
                .packet(CloseStream {}.into())
                .session_state(SessionState::Closing)
                .build()
                .map_err(|_| ());
        }

        if stanza.to.as_ref().map(|t| t.to_string()) != Some("localhost".into()) {
            return response
                .packet(StreamError { kind: StreamErrorKind::HostUnknown }.into())
                .packet(CloseStream {}.into())
                .session_state(SessionState::Closing)
                .build()
                .map_err(|_| ());
        }

        match state {
            SessionState::Opening => {
                response.packet(StreamFeatures { features: Features::StartTls }.into());
            }

            SessionState::Negociated => {
                if let Ok(features) = SessionManager::from_registry().send(GetMechanisms("locahost".into())).await.map_err(|_| ())? {
                    response.packet(StreamFeatures { features }.into()).session_state(SessionState::Authenticating);
                }
            }
            SessionState::Authenticated => {
                response.packet(StreamFeatures { features: Features::Bind }.into()).session_state(SessionState::Binding);
            }
            state => {
                error!("Action({:?}) at this stage isn't possible", state);
                return Self::not_authorized_and_close(&mut response);
            }
        }
        response.build().map_err(|_| ())
    }
}

#[async_trait::async_trait]
impl StanzaHandler<StartTls> for UnauthenticatedSession {
    async fn handle(_state: &SessionState, _stanza: &StartTls) -> Result<SessionManagementPacketResult, ()> {
        let mut response = SessionManagementPacketResultBuilder::default();

        response.session_state(SessionState::Negociating).packet(ProceedTls::default().into());

        response.build().map_err(|_| ())
    }
}

#[async_trait::async_trait]
impl StanzaHandler<Auth> for UnauthenticatedSession {
    async fn handle(_state: &SessionState, stanza: &Auth) -> Result<SessionManagementPacketResult, ()> {
        let mut response = SessionManagementPacketResultBuilder::default();

        let _ = AuthenticationManager::from_registry().send(AuthenticationRequest::new(stanza.clone())).await;
        response.session_state(SessionState::Authenticated).packet(SASLSuccess::default().into());

        response.build().map_err(|_| ())
    }
}

#[async_trait::async_trait]
impl StanzaHandler<Bind> for UnauthenticatedSession {
    async fn handle(_state: &SessionState, _stanza: &Bind) -> Result<SessionManagementPacketResult, ()> {
        let response = SessionManagementPacketResultBuilder::default();

        response.build().map_err(|_| ())
    }
}

#[async_trait::async_trait]
impl StanzaHandler<StreamError> for UnauthenticatedSession {
    async fn handle(_state: &SessionState, _stanza: &StreamError) -> Result<SessionManagementPacketResult, ()> {
        let mut response = SessionManagementPacketResultBuilder::default();

        response.session_state(SessionState::Closing).packet(CloseStream {}.into()).build().map_err(|_| ())
    }
}
