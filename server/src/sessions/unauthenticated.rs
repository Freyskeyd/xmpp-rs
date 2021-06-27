use std::str::FromStr;

use crate::{
    authentication::AuthenticationManager,
    sessions::SessionManagementPacketResultBuilder,
    sessions::{manager::SessionManager, state::SessionState, SessionManagementPacketResult},
    AuthenticationRequest,
};
use actix::{Actor, Context, SystemService};

use jid::{BareJid, Jid};
use log::{error, trace};
use tokio::sync::mpsc::Sender;
use uuid::Uuid;
use xmpp_proto::{
    ns, Auth, Bind, CloseStream, Features, FromXmlElement, GenericIq, IqType, NonStanza, OpenStream, Packet, ProceedTls, SASLSuccess, Stanza, StartTls, StreamError, StreamErrorKind, StreamFeatures,
};
use xmpp_xml::Element;

use super::manager::GetMechanisms;

#[async_trait::async_trait]
pub(crate) trait PacketHandler {
    async fn handle_packet(state: &SessionState, stanza: &Packet, from: Option<Sender<SessionManagementPacketResult>>) -> Result<(), ()>;
}

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
    async fn handle_packet(state: &SessionState, stanza: &Packet, from: Option<Sender<SessionManagementPacketResult>>) -> Result<(), ()> {
        match stanza {
            Packet::NonStanza(stanza) => Self::handle(state, &**stanza).await.map(|res| res.send(from)),
            Packet::Stanza(stanza) => Self::handle(state, &**stanza).await.map(|res| res.send(from)),
            Packet::InvalidPacket(invalid_packet) => {
                let mut response = SessionManagementPacketResultBuilder::default();

                SessionManager::handle_invalid_packet(state, invalid_packet, &mut response).map(|res| res.send(from))
            }
        }
    }
}

#[async_trait::async_trait]
trait StanzaHandler<T> {
    async fn handle(state: &SessionState, stanza: &T) -> Result<SessionManagementPacketResult, ()>;
}

#[async_trait::async_trait]
impl StanzaHandler<Stanza> for UnauthenticatedSession {
    async fn handle(state: &SessionState, stanza: &Stanza) -> Result<SessionManagementPacketResult, ()> {
        let fut = match stanza {
            Stanza::IQ(stanza) => Self::handle(state, stanza),
            _ => Box::pin(async { Err(()) }),
        };

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

            SessionState::Negociated => match SessionManager::from_registry().send(GetMechanisms("locahost".into())).await.map_err(|_| ())? {
                Ok(features) => {
                    response.packet(StreamFeatures { features }.into()).session_state(SessionState::Authenticating);
                }
                Err(_) => {}
            },
            SessionState::Authenticated => {
                response.packet(StreamFeatures { features: Features::Bind }.into()).session_state(SessionState::Binding);
            }
            state => {
                error!("Action({:?}) at this stage isn't possible", state);
                return SessionManager::not_authorized_and_close(&mut response);
            }
        }
        response.build().map_err(|_| ())
    }
}

#[async_trait::async_trait]
impl StanzaHandler<GenericIq> for UnauthenticatedSession {
    async fn handle(state: &SessionState, stanza: &GenericIq) -> Result<SessionManagementPacketResult, ()> {
        let mut response = SessionManagementPacketResultBuilder::default();

        if stanza.get_type() == IqType::Set {
            match state {
                SessionState::Binding => {
                    // We expect a binding command here
                    match stanza.get_element() {
                        Some(element) => {
                            match element.find((ns::BIND, "bind")) {
                                Some(bind_element) => {
                                    let _bindd = Bind::from_element(bind_element);
                                    let mut result_element = Element::new_with_namespaces((ns::STREAM, "iq"), element);

                                    result_element
                                        .set_attr("id", stanza.get_id())
                                        .set_attr("type", "result")
                                        .append_new_child((ns::BIND, "bind"))
                                        .append_new_child((ns::BIND, "jid"))
                                        .set_text(format!("SOME@localhost/{}", ""));

                                    let result = GenericIq::from_element(&result_element).unwrap();
                                    trace!("Respond with : {:?}", result);
                                    // its bind
                                    response.packet(result.into()).session_state(SessionState::Binded);
                                }
                                None => {
                                    trace!("Something failed in Binding");
                                    return Err(());
                                }
                            }
                        }
                        None => {
                            trace!("IQ without element");
                            return Err(());
                        }
                    }
                }
                _ => {
                    trace!("Unsupported state");
                    return Err(());
                }
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
        todo!()
    }
}
