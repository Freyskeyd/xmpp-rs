use crate::{
    sessions::{state::SessionState, AuthenticationRequest, SessionManagementPacket, SessionManagementPacketResultBuilder},
    AuthenticationManager,
};
use actix::{Actor, Context, Handler, Supervised, SystemService};
use log::{error, trace};
use tokio::sync::mpsc::Sender;
use xmpp_proto::{ns, Bind, CloseStream, Features, FromXmlElement, GenericIq, IqType, NonStanza, OpenStream, Packet, ProceedTls, StreamError, StreamErrorKind, StreamFeatures};
use xmpp_xml::Element;

use super::SessionManagementPacketResult;

/// Manage sessions on a node
#[derive(Default)]
pub struct SessionManager {}

impl SessionManager {
    pub(crate) fn new() -> Self {
        Self {}
    }

    fn not_authorized_and_close(mut response: SessionManagementPacketResultBuilder, referer: Sender<SessionManagementPacketResult>) -> Result<(), ()> {
        if let Ok(res) = response
            .packet(StreamError { kind: StreamErrorKind::NotAuthorized }.into())
            .packet(CloseStream {}.into())
            .session_state(SessionState::Closing)
            .build()
        {
            res.send(referer);
        }
        Ok(())
    }

    pub(crate) fn handle_packet(&self, packet: SessionManagementPacket) -> Result<(), ()> {
        trace!("Session manager receive packet");
        let mut response = SessionManagementPacketResultBuilder::default();

        match packet.packet {
            Packet::NonStanza(non_stanza_packet) => match *non_stanza_packet {
                NonStanza::OpenStream(OpenStream { to, lang, version, from, id }) => {
                    response.packet(
                        OpenStream {
                            id,
                            to: from,
                            from: Some("localhost".into()),
                            lang,
                            version,
                        }
                        .into(),
                    );

                    if to != Some("localhost".into()) {
                        if let Ok(res) = response
                            .packet(StreamError { kind: StreamErrorKind::HostUnknown }.into())
                            .packet(CloseStream {}.into())
                            .session_state(SessionState::Closing)
                            .build()
                        {
                            res.send(packet.referer);
                        }
                        return Ok(());
                    }

                    match packet.session_state {
                        SessionState::Opening => {
                            response.packet(StreamFeatures { features: Features::StartTls }.into());
                        }

                        SessionState::Negociated => {
                            response
                                .packet(
                                    StreamFeatures {
                                        features: Features::Mechanisms(vec!["PLAIN".to_string()]),
                                    }
                                    .into(),
                                )
                                .session_state(SessionState::Authenticating);
                        }
                        SessionState::Authenticated => {
                            response.packet(StreamFeatures { features: Features::Bind }.into()).session_state(SessionState::Binding);
                        }
                        state => {
                            error!("Action({:?}) at this stage isn't possible", state);
                            return Self::not_authorized_and_close(response, packet.referer);
                        }
                    }
                }

                NonStanza::StartTls(_) => {
                    response.session_state(SessionState::Negociating).packet(ProceedTls::default().into());
                }

                NonStanza::Auth(e) => {
                    // TODO: Switch to send?
                    AuthenticationManager::from_registry().do_send(AuthenticationRequest::new(e, packet.referer));
                    return Ok(());
                }

                NonStanza::CloseStream(_) => {
                    if let Ok(res) = response.session_state(SessionState::Closing).packet(CloseStream {}.into()).build() {
                        res.send(packet.referer);
                    }
                    return Ok(());
                }
                _ => {
                    trace!("Something failed in manager");
                    return Err(());
                }
            },

            Packet::Stanza(stanza) => match *stanza {
                xmpp_proto::Stanza::IQ(generic_iq) if generic_iq.get_type() == IqType::Set => {
                    match packet.session_state {
                        SessionState::Binding => {
                            // We expect a binding command here
                            match generic_iq.get_element() {
                                Some(element) => {
                                    match element.find((ns::BIND, "bind")) {
                                        Some(bind_element) => {
                                            let _bindd = Bind::from_element(bind_element);
                                            let mut result_element = Element::new_with_namespaces((ns::STREAM, "iq"), element);

                                            result_element
                                                .set_attr("id", generic_iq.get_id())
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
                                            trace!("Something failed in manager");
                                            return Err(());
                                        }
                                    }
                                }
                                None => {
                                    trace!("Something failed in manager");
                                    return Err(());
                                }
                            }
                        }
                        _ => {
                            trace!("Something failed in manager");
                            return Err(());
                        }
                    }
                }
                _ => {
                    // return Self::not_authorized_and_close(response, packet.referer);
                    return Err(());
                }
            },
        }

        if let Ok(res) = response.build() {
            trace!("Sending response to referer");
            res.send(packet.referer);
        }

        Ok(())
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

impl Handler<SessionManagementPacket> for SessionManager {
    type Result = Result<(), ()>;

    fn handle(&mut self, packet: SessionManagementPacket, _ctx: &mut Self::Context) -> Self::Result {
        self.handle_packet(packet)
    }
}
