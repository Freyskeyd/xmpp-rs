use super::session::TcpSession;
use crate::sessions::state::StaticSessionState;
use crate::{config::StartTLSConfig, listeners::tcp::TcpOpenStream};
use crate::{config::TcpListenerConfig, messages::tcp::NewSession};
use crate::{listeners::XmppStreamHolder, parser::codec::XmppCodec, sessions::unauthenticated::UnauthenticatedSession};
use crate::{router::Router, sessions::Session};
use actix::{prelude::*, spawn};
use log::{error, info, trace};
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
    acceptor: Option<TlsAcceptor>,
    sessions: Vec<Addr<TcpSession>>,
}

impl TcpListener {
    pub(crate) fn new(acceptor: Option<TlsAcceptor>) -> Self {
        Self { acceptor, sessions: Vec::new() }
    }

    pub(crate) fn start(config: &TcpListenerConfig, router: Addr<Router>) -> Result<Addr<Self>, ()> {
        let ip = format!("{}:{}", config.ip, config.port);
        let socket_addr = SocketAddr::from_str(&ip).unwrap();

        let acceptor = match config.starttls {
            StartTLSConfig::Unavailable => None,
            StartTLSConfig::Required(ref cert_cfg) | StartTLSConfig::Available(ref cert_cfg) => {
                let cert = Path::new(&cert_cfg.cert_path);
                let keys = Path::new(&cert_cfg.key_path);

                let certs = load_certs(cert).unwrap();
                let mut keys = load_keys(keys).unwrap();

                let mut config = ServerConfig::new(NoClientAuth::new());
                config.set_single_cert(certs, keys.remove(0)).map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err)).unwrap();
                let acceptor = TlsAcceptor::from(Arc::new(config));

                Some(acceptor)
            }
        };

        let addr = Self::create(|_ctx| Self::new(acceptor));
        let tcp_listener = addr.clone();

        trace!("Starting new TCP listener on {} with {}", ip, config.starttls);
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

        Box::pin(
            async move { session.send(TcpOpenStream { stream: msg.0, acceptor }).await.unwrap().map_err(|_| ()) }
                .into_actor(self)
                .map(|res: Result<(XmppStreamHolder, StaticSessionState), ()>, act: &mut TcpListener, _ctx| match res {
                    Ok((stream, state)) => {
                        trace!("Session succeed");
                        let session = TcpSession::create(|ctx| {
                            let (r, w) = tokio::io::split(stream.inner);

                            let session = Session::create(|session_ctx| Session::new(state.clone(), session_ctx.address(), ctx.address().recipient()));
                            TcpSession::add_stream(FramedRead::new(r, XmppCodec::new()), ctx);
                            TcpSession::new(state, 0, actix::io::FramedWrite::new(Box::pin(w), XmppCodec::new(), ctx), session)
                        });

                        act.sessions.push(session)
                    }

                    Err(e) => {
                        error!("Session failed {:?}", e);
                    }
                })
                .map(|_, _, _| {
                    trace!("Session killed");
                }),
        )
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
