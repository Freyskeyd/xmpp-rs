use crate::{tcp::TcpSession, XmppCodec};
use std::{
    fs::File,
    io::{self, BufReader},
    net::SocketAddr,
    path::Path,
    str::FromStr,
    sync::Arc,
};

use actix::{prelude::*, spawn};
use bytes::BytesMut;
use log::info;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};
use tokio_rustls::{
    rustls::{
        internal::pemfile::{certs, pkcs8_private_keys},
        Certificate, NoClientAuth, PrivateKey, ServerConfig,
    },
    TlsAcceptor,
};
use tokio_util::codec::{Decoder, Encoder, FramedRead};
use xmpp_proto::{Features, NonStanza, OpenStream, Packet, ProceedTls, StreamFeatures};

use crate::router::Router;

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

        let mut stream = msg.0;
        let router = msg.2.clone();

        let acceptor = self.acceptor.clone();

        let fut = async move {
            let mut buf = BytesMut::with_capacity(4096);
            let mut codec = XmppCodec::new();
            loop {
                stream.readable().await.unwrap();

                match stream.read_buf(&mut buf).await {
                    Ok(0) => {}
                    Ok(_) => {
                        if let Ok(Some(packet)) = codec.decode(&mut buf) {
                            println!("PACKET: {:?}", packet);
                            match packet {
                                Packet::NonStanza(NonStanza::OpenStream(OpenStream { to, xmlns, lang, version, from, id })) => {
                                    let _ = codec.encode(
                                        Packet::NonStanza(NonStanza::OpenStream(OpenStream {
                                            id,
                                            to: from,
                                            from: to,
                                            xmlns,
                                            lang,
                                            version,
                                        })),
                                        &mut buf,
                                    );
                                    let _ = codec.encode(Packet::NonStanza(NonStanza::StreamFeatures(StreamFeatures { features: Features::StartTlsInit })), &mut buf);
                                    let _ = stream.write_buf(&mut buf).await;
                                    continue;
                                }
                                Packet::NonStanza(NonStanza::StartTls(_)) => {
                                    let _ = codec.encode(Packet::NonStanza(NonStanza::ProceedTls(ProceedTls::default())), &mut buf);
                                    let _ = stream.write_buf(&mut buf).await;
                                    let _ = stream.flush().await;
                                    break;
                                }
                                Packet::NonStanza(e) => {
                                    let _ = codec.encode(Packet::NonStanza(e), &mut buf);
                                    let _ = stream.write_buf(&mut buf).await;
                                    let _ = stream.flush().await;

                                    break;
                                }
                                _ => continue,
                            }
                        }
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                        continue;
                    }
                    Err(e) => {
                        // return Err(e.into());
                        println!("err: {:?}", e);
                        break;
                    }
                }
            }

            acceptor.accept(stream).await.unwrap()
        }
        .into_actor(self)
        .map(|stream, act, _ctx| {
            let session = TcpSession::create(|ctx| {
                let (r, w) = tokio::io::split(stream);

                TcpSession::add_stream(FramedRead::new(r, XmppCodec::new()), ctx);
                TcpSession::new(0, router, actix::io::FramedWrite::new(Box::pin(w), XmppCodec::new(), ctx))
            });

            act.sessions.push(session)
        });

        Box::pin(fut)
    }
}

struct TcpSocket {}
impl Actor for TcpSocket {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {}
}

// Move to utils?
fn load_certs(path: &Path) -> io::Result<Vec<Certificate>> {
    certs(&mut BufReader::new(File::open(path)?)).map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid cert"))
}

fn load_keys(path: &Path) -> io::Result<Vec<PrivateKey>> {
    let f = File::open(path)?;
    pkcs8_private_keys(&mut BufReader::new(f)).map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid key"))
}

#[derive(Message)]
#[rtype("()")]
pub struct NewSession(pub TcpStream, pub std::net::SocketAddr, pub Addr<Router>);
