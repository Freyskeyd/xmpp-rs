use crate::{
    messages::{SessionManagementPacketError, SessionManagementPacketResult, SessionManagementPacketResultBuilder, SessionPacket},
    packet::{PacketHandler, StanzaHandler},
    sessions::state::SessionState,
};
use actix::prelude::*;
use jid::{BareJid, Jid};
use log::{error, trace};
use std::str::FromStr;
use uuid::Uuid;
use xmpp_proto::{ns, Bind, CloseStream, Features, FromXmlElement, GenericIq, IqType, NonStanza, OpenStream, Packet, Stanza, StreamError, StreamErrorKind, StreamFeatures};
use xmpp_xml::Element;

pub(crate) mod manager;
pub(crate) mod state;
pub(crate) mod unauthenticated;

/// Hold a session on a node
pub struct Session {
    pub(crate) state: SessionState,
    pub(crate) sink: Recipient<SessionManagementPacketResult>,
}

impl Session {}
impl Actor for Session {
    type Context = Context<Self>;
}

impl Handler<SessionPacket> for Session {
    type Result = ResponseActFuture<Self, Result<(), ()>>;

    fn handle(&mut self, msg: SessionPacket, _ctx: &mut Self::Context) -> Self::Result {
        let state = self.state;
        let fut = async move {
            println!("Handle packet in session");
            Self::handle_packet(&state, &msg.packet, ()).await
        };

        Box::pin(fut.into_actor(self).map(|res, act, _ctx| match res {
            Ok(result) => {
                act.state = result.session_state;

                // TODO: Handle better
                let _ = act.sink.try_send(result);
                Ok(())
            }
            Err(_) => Err(()),
        }))
    }
}

#[async_trait::async_trait]
impl PacketHandler for Session {
    type Result = Result<SessionManagementPacketResult, SessionManagementPacketError>;
    type From = ();

    async fn handle_packet(state: &SessionState, stanza: &Packet, _from: Self::From) -> Self::Result {
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
impl StanzaHandler<Stanza> for Session {
    async fn handle(state: &SessionState, stanza: &Stanza) -> Result<SessionManagementPacketResult, SessionManagementPacketError> {
        let fut = match stanza {
            Stanza::IQ(stanza) => <Self as StanzaHandler<_>>::handle(state, stanza),
            _ => Box::pin(async { Err(SessionManagementPacketError::Unknown) }),
        };

        fut.await
    }
}
#[async_trait::async_trait]
impl StanzaHandler<NonStanza> for Session {
    async fn handle(state: &SessionState, stanza: &NonStanza) -> Result<SessionManagementPacketResult, SessionManagementPacketError> {
        let fut = match stanza {
            NonStanza::OpenStream(stanza) => <Self as StanzaHandler<_>>::handle(state, stanza),
            // NonStanza::StartTls(stanza) => Self::handle(state, stanza),
            // NonStanza::Auth(stanza) => Self::handle(state, stanza),
            // NonStanza::StreamError(stanza) => Self::handle(state, stanza),
            // NonStanza::CloseStream(stanza) => Self::handle(state, stanza),
            _ => Box::pin(async { Err(SessionManagementPacketError::Unknown) }),
        };

        fut.await
    }
}

#[async_trait::async_trait]
impl StanzaHandler<OpenStream> for Session {
    async fn handle(state: &SessionState, stanza: &OpenStream) -> Result<SessionManagementPacketResult, SessionManagementPacketError> {
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

        match state {
            SessionState::Opening => {
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
impl StanzaHandler<GenericIq> for Session {
    async fn handle(state: &SessionState, stanza: &GenericIq) -> Result<SessionManagementPacketResult, SessionManagementPacketError> {
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
                                    return Err(SessionManagementPacketError::Unknown);
                                }
                            }
                        }
                        None => {
                            trace!("IQ without element");
                            return Err(SessionManagementPacketError::Unknown);
                        }
                    }
                }
                _ => {
                    trace!("Unsupported state");
                    return Err(SessionManagementPacketError::Unknown);
                }
            }
        }

        Ok(response.build()?)
    }
}
