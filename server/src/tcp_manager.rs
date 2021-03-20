use actix::prelude::*;
use bytes::BytesMut;
use std::io;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};
use tokio_rustls::TlsAcceptor;
use tokio_util::codec::{Decoder, Encoder, FramedRead};

use crate::{router::Router, tcp::TcpSession, XmppCodec};
use xmpp_proto::{Features, NonStanza, OpenStream, Packet, ProceedTls, StreamFeatures};

pub(crate) struct TcpManager {
    acceptor: TlsAcceptor,
    #[allow(dead_code)]
    sessions: Vec<TcpSession>,
}

impl TcpManager {
    pub(crate) fn new(acceptor: TlsAcceptor) -> Self {
        Self { acceptor, sessions: vec![] }
    }
}

impl Actor for TcpManager {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {}
}

impl Handler<NewSession> for TcpManager {
    type Result = ResponseActFuture<Self, ()>;
    fn handle(&mut self, msg: NewSession, _ctx: &mut Self::Context) -> Self::Result {
        println!("New Session asked");
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
        .map(|stream, _act, _ctx| {
            TcpSession::create(|ctx| {
                let (r, w) = tokio::io::split(stream);

                TcpSession::add_stream(FramedRead::new(r, XmppCodec::new()), ctx);
                TcpSession::new(0, router, actix::io::FramedWrite::new(w, XmppCodec::new(), ctx))
            });
        });

        Box::pin(fut)
    }
}

#[derive(Message)]
#[rtype("()")]
pub struct NewSession(pub TcpStream, pub std::net::SocketAddr, pub Addr<Router>);
