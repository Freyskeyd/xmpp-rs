use actix::spawn;
use actix::{Actor, Addr, StreamHandler};
use actix_web::{App, HttpServer};
use parser::XmppCodec;
use router::Router;
use std::{
    fs::File,
    io::{self, BufReader},
    net::SocketAddr,
    path::Path,
    str::FromStr,
    sync::Arc,
};
use tcp_manager::{NewSession, TcpManager};
use tokio::net::TcpListener;
use tokio_rustls::{
    rustls::{
        internal::pemfile::{certs, pkcs8_private_keys},
        Certificate, NoClientAuth, PrivateKey, ServerConfig,
    },
    TlsAcceptor,
};

mod parser;
mod router;
mod tcp;
mod tcp_manager;

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
        tcp_server("", router, Path::new(&self.cert.unwrap()), Path::new(&self.keys.unwrap()));
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
        // Starting s2s_in
        // Starting s2s_out
        // Starting s2s
        // Starting service
        // Starting captcha
        // Starting ext_mod
        // Starting acme
        // Starting auth
        // Starting oauth

        // std::thread::spawn(move || {
        //     let mut sys = actix_web::rt::System::new();
        //     // let srv = HttpServer::new(|| App::new().route("/ws/", web::get().to(index))).bind("127.0.0.1:8080").unwrap().run();
        //     let srv = HttpServer::new(|| App::new()).bind("127.0.0.1:8080").unwrap().run();
        //     let _ = sys.block_on(srv);
        // });

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
fn load_certs(path: &Path) -> io::Result<Vec<Certificate>> {
    certs(&mut BufReader::new(File::open(path)?)).map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid cert"))
}

fn load_keys(path: &Path) -> io::Result<Vec<PrivateKey>> {
    let f = File::open(path)?;
    pkcs8_private_keys(&mut BufReader::new(f)).map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid key"))
}

/// Define tcp server that will accept incoming tcp connection and create
/// chat actors.
// pub fn tcp_server(_s: &str, server: Addr<ChatServer>) {
fn tcp_server(_s: &str, router: Addr<Router>, cert: &Path, keys: &Path) {
    // Create server listener
    let addr = SocketAddr::from_str("127.0.0.1:5222").unwrap();

    let certs = load_certs(cert).unwrap();
    let mut keys = load_keys(keys).unwrap();

    let mut config = ServerConfig::new(NoClientAuth::new());
    config.set_single_cert(certs, keys.remove(0)).map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err)).unwrap();
    let acceptor = TlsAcceptor::from(Arc::new(config));

    let manager = TcpManager::create(|_ctx| TcpManager::new(acceptor));

    spawn(async move {
        // Openning TCP to prepare for STARTLS
        let listener = TcpListener::bind(&addr).await.unwrap();

        while let Ok((stream, socket_addr)) = listener.accept().await {
            manager.do_send(NewSession(stream, socket_addr, router.clone()));
        }
    });
}
