use actix::prelude::*;
use actix::{Actor, StreamHandler};
use actix_codec::AsyncWrite;
use actix_web::{App, HttpServer};
use parser::XmppCodec;
use router::Router;
use std::path::Path;
use xmpp_proto::{ns, Features, FromXmlElement, GenericIq, IqType, NonStanza, OpenStream, Packet, ProceedTls, StreamFeatures};
use xmpp_xml::Element;

mod listeners;
mod parser;
mod router;

pub struct Server {}

impl Server {
    pub fn build() -> ServerBuilder {
        ServerBuilder::default()
    }
}

#[derive(Default)]
pub struct ServerBuilder {
    cert: Option<String>,
    keys: Option<String>,
}

impl ServerBuilder {
    pub fn cert<T: Into<String>>(mut self, cert: T) -> Self {
        self.cert = Some(cert.into());

        self
    }

    pub fn keys<T: Into<String>>(mut self, keys: T) -> Self {
        self.keys = Some(keys.into());

        println!("{:?}", self.keys);

        self
    }

    pub async fn launch(self) -> std::io::Result<()> {
        // Starting systemd
        // Starting hooks
        // Starting clustering
        // Starting translation
        // Starting access permissions
        // Starting ctl
        // Starting commands
        // Starting admin
        // Starting Router
        let router = Router::new().start();
        // Starting all listener (tcp, ws)
        let _tcp_listener = crate::listeners::tcp::TcpListener::start("", router, Path::new(&self.cert.unwrap()), Path::new(&self.keys.unwrap()));
        // Starting pkix
        // Starting ACL
        // Starting Shaper
        // Starting DB
        // Starting Backend
        // Starting Sql
        // Starting IQ
        // Starting router multicast
        // Starting local
        // Starting Session Manager
        SessionManager::new().start();
        // Starting s2s_in
        // Starting s2s_out
        // Starting s2s
        // Starting service
        // Starting captcha
        // Starting ext_mod
        // Starting acme
        // Starting auth
        // Starting oauth

        std::thread::spawn(move || {
            let sys = actix_web::rt::System::new();
            let _ = sys.block_on(async { HttpServer::new(|| App::new().route("/ws", web::get().to(index))).bind("127.0.0.1:8080").unwrap().run().await });
        });

        // Start API

        tokio::signal::ctrl_c().await.unwrap();
        println!("Ctrl-C received, shutting down");

        Ok(())
    }
}
use actix::ActorContext;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;

/// websocket connection is long running connection, it easier
/// to handle with an actor
struct MyWebSocket {
    // hb: Instant,
}

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;

    // fn started(&mut self, ctx: &mut Self::Context) {
    //     self.hb(ctx);
    // }
}

/// Handler for `ws::Message`
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        // process websocket messages
        println!("WS: {:?}", msg);
        match msg {
            // Ok(ws::Message::Ping(msg)) => {
            //     self.hb = Instant::now();
            //     ctx.pong(&msg);
            // }
            // Ok(ws::Message::Pong(_)) => {
            //     self.hb = Instant::now();
            // }
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}

async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(MyWebSocket {}, &req, stream);
    println!("{:?}", resp);
    resp
}

// /// Hold a session on a node
pub struct Session {
    #[allow(dead_code)]
    sink: Box<dyn AsyncWrite>,
}

impl Session {}
// /// Manage sessions on a node
#[derive(Default)]
pub struct SessionManager {}
impl SessionManager {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl Supervised for SessionManager {}

impl SystemService for SessionManager {}
impl Actor for SessionManager {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("SessionManager started");
    }
}

#[derive(Debug, Message, Clone)]
#[rtype(result = "Result<SessionManagementPacketResult, ()>")]
struct SessionManagementPacket {
    session_state: SessionState,
    packet: Packet,
}

#[derive(derive_builder::Builder, Debug, Clone)]
#[builder(setter(into))]
struct SessionManagementPacketResult {
    #[builder(default = "SessionState::Opening")]
    session_state: SessionState,
    #[builder(setter(each = "packet", into = "true"))]
    packets: Vec<Packet>,
}

#[derive(Debug, Clone, Copy)]
enum SessionState {
    Opening,
    Negociating,
    Negociated,
    Authenticating,
    Authenticated,
    Binding,
}

impl Handler<SessionManagementPacket> for SessionManager {
    type Result = Result<SessionManagementPacketResult, ()>;

    fn handle(&mut self, packet: SessionManagementPacket, _ctx: &mut Self::Context) -> Self::Result {
        let mut response = SessionManagementPacketResultBuilder::default();
        match packet.packet {
            Packet::NonStanza(non_stanza_packet) => match *non_stanza_packet {
                NonStanza::OpenStream(OpenStream { to, lang, version, from, id }) => {
                    response.packet(
                        OpenStream {
                            id,
                            to: from,
                            from: to,
                            lang,
                            version,
                        }
                        .into(),
                    );

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
                        SessionState::Negociating => return Err(()),
                        SessionState::Authenticating => return Err(()),
                        SessionState::Binding => return Err(()),
                    }
                }

                NonStanza::StartTls(_) => {
                    response.session_state(SessionState::Negociating).packet(ProceedTls::default().into());
                }

                NonStanza::Auth(_) => {
                    // Authentification Async?
                    response.session_state(SessionState::Authenticated).packet(Packet::NonStanza(Box::new(NonStanza::SASLSuccess)));
                }
                _ => return Err(()),
            },

            Packet::Stanza(stanza) => match *stanza {
                xmpp_proto::Stanza::IQ(generic_iq) if generic_iq.get_type() == IqType::Set => {
                    match packet.session_state {
                        SessionState::Binding => {
                            // We expect a binding command here
                            match generic_iq.get_element() {
                                Some(element) => {
                                    match element.find((ns::BIND, "bind")) {
                                        Some(_) => {
                                            let mut result_element = Element::new_with_namespaces((ns::STREAM, "iq"), element);

                                            result_element
                                                .set_attr("id", generic_iq.get_id())
                                                .set_attr("type", "result")
                                                .append_new_child((ns::BIND, "bind"))
                                                .append_new_child((ns::BIND, "jid"))
                                                .set_text("SOME@localhost");

                                            let result = GenericIq::from_element(result_element).unwrap();
                                            println!("Respond with : {:?}", result);
                                            // its bind
                                            response.packet(result.into());
                                        }
                                        None => return Err(()),
                                    }
                                }
                                None => return Err(()),
                            }
                        }
                        _ => return Err(()),
                    }
                }
                _ => return Err(()),
            },
        }

        response.build().map_err(|_| ())
    }
}

// /// Manage listeners on a node
// pub struct Listeners {}
// /// Listen for TCP on a node
// pub struct TcpListener {}
// /// Listen for WS on a node
// pub struct WsListener {}
// /// Hold a TCP session on a node
// pub struct TcpSession {}
// /// Hold a WS session on a node
// pub struct WsSession {}
// pub trait PacketReceiver {}
