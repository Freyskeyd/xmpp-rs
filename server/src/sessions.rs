use crate::messages::system::SessionCommand;
use crate::messages::system::SessionCommandAction;
use crate::sessions::state::StaticSessionState;
use crate::{
    messages::{system::RegisterSession, system::UnregisterSession, SessionManagementPacketError, SessionManagementPacketResult, SessionManagementPacketResultBuilder, SessionPacket},
    packet::{PacketHandler, StanzaHandler},
    sessions::manager::SessionManager,
    sessions::state::SessionState,
};
use actix::prelude::*;
use jid::{BareJid, FullJid, Jid};
use log::{error, trace};
use std::convert::TryInto;
use std::str::FromStr;
use std::time::Duration;
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
    timeout_handler: Option<SpawnHandle>,
    jid: Jid,
    self_addr: Addr<Self>,
}

impl Session {
    pub(crate) fn new(state: StaticSessionState, self_addr: Addr<Self>, sink: Recipient<SessionManagementPacketResult>) -> Self {
        Self {
            state: SessionState::Opening,
            sink,
            timeout_handler: None,
            jid: state.jid.unwrap(),
            self_addr,
        }
    }

    pub(crate) fn sync_state(&mut self, StaticSessionState { state, jid, .. }: &StaticSessionState) {
        self.state = state.clone();
        if let Some(jid) = jid {
            self.jid = jid.clone();
        }
    }

    fn reset_timeout(&mut self, ctx: &mut <Self as Actor>::Context) {
        if let Some(handler) = self.timeout_handler {
            if ctx.cancel_future(handler) {
                trace!("Timeout handler resetted for session");
            } else {
                trace!("Unable to reset timeout handler for session");
                ctx.set_mailbox_capacity(0);
                ctx.notify(SessionCommand(SessionCommandAction::Kill));
                return ();
            }
        }

        let referer = ctx.address();
        self.timeout_handler = Some(ctx.run_later(Duration::from_secs(10), move |actor, ctx| {
            println!("Duration ended");
            let fut = referer.send(SessionCommand(SessionCommandAction::Kill)).into_actor(actor).map(|_, _, _| ());

            ctx.spawn(fut);
        }));
    }
}

impl TryInto<StaticSessionState> for &mut Session {
    type Error = ();

    fn try_into(self) -> Result<StaticSessionState, Self::Error> {
        let addr = self.self_addr.clone().recipient::<SessionCommand>();
        StaticSessionState::builder().state(self.state).jid(self.jid.clone()).addr_session_command(addr).build().map_err(|_| ())
    }
}

impl Actor for Session {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        trace!("Starting Session");
    }

    fn stopping(&mut self, _ctx: &mut Self::Context) -> actix::Running {
        trace!("Stopping Session");
        let _ = SessionManager::from_registry().try_send(UnregisterSession { jid: self.jid.clone() });
        actix::Running::Stop
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        trace!("Session Stopped");
    }
}

impl Handler<SessionCommand> for Session {
    type Result = Result<(), ()>;

    fn handle(&mut self, msg: SessionCommand, ctx: &mut Self::Context) -> Self::Result {
        match msg.0 {
            SessionCommandAction::Kill => {
                if let Ok(result) = Self::close(&mut SessionManagementPacketResultBuilder::default()) {
                    if let Some(ref state) = result.real_session_state {
                        self.sync_state(state);
                    }

                    let _ = self.sink.try_send(result);
                    ctx.stop();
                }
                Ok(())
            }
        }
    }
}

impl Handler<SessionPacket> for Session {
    type Result = ResponseActFuture<Self, Result<(), ()>>;

    fn handle(&mut self, msg: SessionPacket, ctx: &mut Self::Context) -> Self::Result {
        self.reset_timeout(ctx);
        let state = self.try_into().unwrap();
        let fut = async move {
            println!("Handle packet in session");
            Self::handle_packet(state, &msg.packet, ()).await
        };

        Box::pin(fut.into_actor(self).map(|res, act, _ctx| match res {
            Ok(result) => {
                if let Some(ref r_state) = result.real_session_state {
                    act.sync_state(r_state);
                }

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

    async fn handle_packet(state: StaticSessionState, stanza: &Packet, _from: Self::From) -> Self::Result {
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
    async fn handle(state: StaticSessionState, stanza: &Stanza) -> Result<SessionManagementPacketResult, SessionManagementPacketError> {
        let fut = match stanza {
            Stanza::IQ(stanza) => <Self as StanzaHandler<_>>::handle(state, stanza),
            _ => Box::pin(async { Err(SessionManagementPacketError::Unknown) }),
        };

        fut.await
    }
}
#[async_trait::async_trait]
impl StanzaHandler<NonStanza> for Session {
    async fn handle(state: StaticSessionState, stanza: &NonStanza) -> Result<SessionManagementPacketResult, SessionManagementPacketError> {
        let fut = match stanza {
            NonStanza::OpenStream(stanza) => <Self as StanzaHandler<_>>::handle(state, stanza),
            // NonStanza::StartTls(stanza) => Self::handle(state, stanza),
            // NonStanza::Auth(stanza) => Self::handle(state, stanza),
            // NonStanza::StreamError(stanza) => Self::handle(state, stanza),
            NonStanza::CloseStream(stanza) => <Self as StanzaHandler<_>>::handle(state, stanza),
            _ => Box::pin(async { Err(SessionManagementPacketError::Unknown) }),
        };

        fut.await
    }
}

#[async_trait::async_trait]
impl StanzaHandler<CloseStream> for Session {
    async fn handle(_state: StaticSessionState, _stanza: &CloseStream) -> Result<SessionManagementPacketResult, SessionManagementPacketError> {
        Ok(SessionManagementPacketResultBuilder::default()
            .session_state(SessionState::Closing)
            .packet(CloseStream {}.into())
            .build()?)
    }
}

#[async_trait::async_trait]
impl StanzaHandler<OpenStream> for Session {
    async fn handle(state: StaticSessionState, stanza: &OpenStream) -> Result<SessionManagementPacketResult, SessionManagementPacketError> {
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
    async fn handle(state: StaticSessionState, stanza: &GenericIq) -> Result<SessionManagementPacketResult, SessionManagementPacketError> {
        let mut response = SessionManagementPacketResultBuilder::default();

        if stanza.get_type() == IqType::Set {
            match state.state {
                SessionState::Binding => {
                    // We expect a binding command here
                    match stanza.get_element() {
                        Some(element) => {
                            match element.find((ns::BIND, "bind")) {
                                Some(bind_element) => {
                                    let bindd = Bind::from_element(bind_element).unwrap();
                                    let old_jid = state.jid.clone().unwrap();
                                    let jid = FullJid::new(old_jid.clone().node().unwrap(), old_jid.domain(), bindd.resource.unwrap());

                                    match SessionManager::from_registry()
                                        .send(RegisterSession {
                                            jid: Jid::Full(jid.clone()),
                                            referer: state.get_addr().unwrap(),
                                        })
                                        .await
                                        .unwrap()
                                    {
                                        Ok(_) => {
                                            let mut result_element = Element::new_with_namespaces((ns::STREAM, "iq"), element);

                                            result_element
                                                .set_attr("id", stanza.get_id())
                                                .set_attr("type", "result")
                                                .append_new_child((ns::BIND, "bind"))
                                                .append_new_child((ns::BIND, "jid"))
                                                .set_text(jid.to_string());

                                            let result = GenericIq::from_element(&result_element).unwrap();
                                            trace!("Respond with : {:?}", result);
                                            // its bind
                                            response
                                                .real_session_state(state.clone().set_jid(Jid::Full(jid.clone())))
                                                .packet(result.into())
                                                .session_state(SessionState::Binded);
                                        }
                                        Err(_) => {
                                            trace!("Error binding session");
                                            return Err(SessionManagementPacketError::Unknown);
                                        }
                                    }
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
