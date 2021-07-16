use crate::messages::{system::GetMechanisms, SessionManagementPacketError};
use crate::{
    authentication::AuthenticationManager,
    packet::{PacketHandler, StanzaHandler},
    sessions::SessionManagementPacketResultBuilder,
    sessions::{manager::SessionManager, state::SessionState, SessionManagementPacketResult},
    AuthenticationRequest,
};
use actix::{Actor, Context, SystemService};
use jid::{BareJid, FullJid, Jid};
use log::{error, trace};
use std::str::FromStr;
use uuid::Uuid;
use xmpp_proto::{Auth, Bind, CloseStream, Features, NonStanza, OpenStream, Packet, ProceedTls, SASLSuccess, Stanza, StartTls, StreamError, StreamErrorKind, StreamFeatures};

use super::state::StaticSessionState;

#[derive(Default)]
pub(crate) struct UnauthenticatedSession {
    #[allow(dead_code)]
    pub(crate) state: SessionState,
    #[allow(dead_code)]
    pub(crate) jid: Option<FullJid>,
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
    type Result = Result<SessionManagementPacketResult, SessionManagementPacketError>;

    async fn handle_packet(state: StaticSessionState, stanza: &Packet) -> Self::Result {
        match stanza {
            Packet::NonStanza(stanza) => <Self as StanzaHandler<_>>::handle(state, &**stanza).await,
            Packet::Stanza(stanza) => <Self as StanzaHandler<_>>::handle(state, &**stanza).await,
            Packet::InvalidPacket(invalid_packet) => {
                let mut response = SessionManagementPacketResultBuilder::default();

                Self::handle_invalid_packet(state, invalid_packet, &mut response)
            }
        }
    }
}

#[async_trait::async_trait]
impl StanzaHandler<Stanza> for UnauthenticatedSession {
    async fn handle(_state: StaticSessionState, _stanza: &Stanza) -> Result<SessionManagementPacketResult, SessionManagementPacketError> {
        Box::pin(async { Err(SessionManagementPacketError::Unknown) }).await
    }
}

#[async_trait::async_trait]
impl StanzaHandler<NonStanza> for UnauthenticatedSession {
    async fn handle(state: StaticSessionState, stanza: &NonStanza) -> Result<SessionManagementPacketResult, SessionManagementPacketError> {
        match stanza {
            NonStanza::OpenStream(stanza) => <Self as StanzaHandler<_>>::handle(state, stanza),
            NonStanza::StartTls(stanza) => <Self as StanzaHandler<_>>::handle(state, stanza),
            NonStanza::Auth(stanza) => <Self as StanzaHandler<_>>::handle(state, stanza),
            NonStanza::StreamError(stanza) => <Self as StanzaHandler<_>>::handle(state, stanza),
            NonStanza::CloseStream(stanza) => <Self as StanzaHandler<_>>::handle(state, stanza),
            _ => Box::pin(async { Err(SessionManagementPacketError::Unknown) }),
        }
        .await
    }
}

#[async_trait::async_trait]
impl StanzaHandler<CloseStream> for UnauthenticatedSession {
    async fn handle(_state: StaticSessionState, _stanza: &CloseStream) -> Result<SessionManagementPacketResult, SessionManagementPacketError> {
        Ok(SessionManagementPacketResultBuilder::default()
            .session_state(SessionState::Closing)
            .packet(CloseStream {}.into())
            .build()?)
    }
}

#[async_trait::async_trait]
impl StanzaHandler<OpenStream> for UnauthenticatedSession {
    async fn handle(state: StaticSessionState, stanza: &OpenStream) -> Result<SessionManagementPacketResult, SessionManagementPacketError> {
        let mut response = SessionManagementPacketResultBuilder::default();

        response
            .packet(
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
            )
            .session_state(state.state);

        if SessionState::UnsupportedEncoding.eq(&state.state) {
            return Ok(response
                .packet(
                    StreamError {
                        kind: StreamErrorKind::UnsupportedEncoding,
                    }
                    .into(),
                )
                .packet(CloseStream {}.into())
                .session_state(SessionState::Closing)
                .build()?);
        }

        if stanza.version != "1.0" {
            return Ok(response
                .packet(
                    StreamError {
                        kind: StreamErrorKind::UnsupportedVersion,
                    }
                    .into(),
                )
                .packet(CloseStream {}.into())
                .session_state(SessionState::Closing)
                .build()?);
        }

        if stanza.to.as_ref().map(|t| t.to_string()) != Some("localhost".into()) {
            return Ok(response
                .packet(StreamError { kind: StreamErrorKind::HostUnknown }.into())
                .packet(CloseStream {}.into())
                .session_state(SessionState::Closing)
                .build()?);
        }

        match state.state {
            SessionState::Opening => {
                response.packet(StreamFeatures { features: Features::StartTls }.into());
            }

            SessionState::Negociated => {
                if let Ok(features) = SessionManager::from_registry().send(GetMechanisms("locahost".into())).await? {
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

        Ok(response.build()?)
    }
}

#[async_trait::async_trait]
impl StanzaHandler<StartTls> for UnauthenticatedSession {
    async fn handle(_state: StaticSessionState, _stanza: &StartTls) -> Result<SessionManagementPacketResult, SessionManagementPacketError> {
        Ok(SessionManagementPacketResultBuilder::default()
            .session_state(SessionState::Negociating)
            .packet(ProceedTls::default().into())
            .build()?)
    }
}

#[async_trait::async_trait]
impl StanzaHandler<Auth> for UnauthenticatedSession {
    async fn handle(_state: StaticSessionState, stanza: &Auth) -> Result<SessionManagementPacketResult, SessionManagementPacketError> {
        match AuthenticationManager::from_registry().send(AuthenticationRequest::new(stanza.clone())).await.unwrap() {
            Ok(jid) => Ok(SessionManagementPacketResultBuilder::default()
                .session_state(StaticSessionState::builder().jid(Some(jid)).state(SessionState::Authenticated).build().unwrap())
                .packet(SASLSuccess::default().into())
                .build()?),
            Err(_) => Err(SessionManagementPacketError::Unknown),
        }
    }
}

#[async_trait::async_trait]
impl StanzaHandler<Bind> for UnauthenticatedSession {
    async fn handle(_state: StaticSessionState, _stanza: &Bind) -> Result<SessionManagementPacketResult, SessionManagementPacketError> {
        Ok(SessionManagementPacketResultBuilder::default().build()?)
    }
}

#[async_trait::async_trait]
impl StanzaHandler<StreamError> for UnauthenticatedSession {
    async fn handle(_state: StaticSessionState, _stanza: &StreamError) -> Result<SessionManagementPacketResult, SessionManagementPacketError> {
        Ok(SessionManagementPacketResultBuilder::default()
            .session_state(SessionState::Closing)
            .packet(CloseStream {}.into())
            .build()?)
    }
}
