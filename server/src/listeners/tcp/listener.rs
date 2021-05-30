use super::session::TcpSession;
use crate::listeners::tcp::NewSession;
use crate::listeners::tcp::TcpOpenStream;
use crate::router::Router;
use crate::{listeners::XmppStream, parser::codec::XmppCodec, sessions::unauthenticated::UnauthenticatedSession};
use actix::{prelude::*, spawn};
use log::info;
use std::{
    fs::File,
    io::{self, BufReader},
    net::SocketAddr,
    path::Path,
    str::FromStr,
    sync::Arc,
};
use tokio_rustls::{
    rustls::{
        internal::pemfile::{certs, pkcs8_private_keys},
        Certificate, NoClientAuth, PrivateKey, ServerConfig,
    },
    TlsAcceptor,
};
use tokio_util::codec::FramedRead;

pub(crate) struct TcpListener {
    acceptor: TlsAcceptor,
    sessions: Vec<Addr<TcpSession>>,
}

impl TcpListener {
    pub(crate) fn new(acceptor: TlsAcceptor) -> Self {
        Self { acceptor, sessions: Vec::new() }
    }

    pub(crate) fn start(_s: &str, router: Addr<Router>, cert: &Path, keys: &Path) -> Result<Addr<Self>, ()> {
        // Create server listener
        let socket_addr = SocketAddr::from_str("127.0.0.1:5222").unwrap();

        let certs = load_certs(cert).unwrap();
        let mut keys = load_keys(keys).unwrap();

        let mut config = ServerConfig::new(NoClientAuth::new());
        config.set_single_cert(certs, keys.remove(0)).map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err)).unwrap();
        let acceptor = TlsAcceptor::from(Arc::new(config));

        let addr = Self::create(|_ctx| Self::new(acceptor));
        let tcp_listener = addr.clone();

        spawn(async move {
            // Openning TCP to prepare for STARTLS
            let listener = tokio::net::TcpListener::bind(&socket_addr).await.unwrap();

            while let Ok((stream, socket_addr)) = listener.accept().await {
                tcp_listener.do_send(NewSession(stream, socket_addr, router.clone()));
            }
        });

        Ok(addr)
    }
}

impl Actor for TcpListener {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {}
}

impl Handler<NewSession> for TcpListener {
    type Result = ResponseActFuture<Self, ()>;
    fn handle(&mut self, msg: NewSession, _ctx: &mut Self::Context) -> Self::Result {
        info!("New TCP Session asked");

        let router = msg.2.clone();

        let acceptor = self.acceptor.clone();

        let session = UnauthenticatedSession::default().start();

        let fut = async move { session.send(TcpOpenStream { stream: msg.0, acceptor }).await.unwrap().unwrap() }
            .into_actor(self)
            .map(|stream: XmppStream, act, _ctx| {
                let session = TcpSession::create(|ctx| {
                    let (r, w) = tokio::io::split(stream.inner);

                    TcpSession::add_stream(FramedRead::new(r, XmppCodec::new()), ctx);
                    TcpSession::new(0, router, actix::io::FramedWrite::new(Box::pin(w), XmppCodec::new(), ctx))
                });

                act.sessions.push(session)
            });

        Box::pin(fut)
    }
}

// Move to utils?
fn load_certs(path: &Path) -> io::Result<Vec<Certificate>> {
    certs(&mut BufReader::new(File::open(path)?)).map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid cert"))
}

fn load_keys(path: &Path) -> io::Result<Vec<PrivateKey>> {
    let f = File::open(path)?;
    pkcs8_private_keys(&mut BufReader::new(f)).map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid key"))
}
