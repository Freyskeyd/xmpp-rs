// TCP
pub(crate) mod tcp;
// WS
mod ws;

use actix::{io::FramedWrite, Actor, Addr, Context, StreamHandler, SystemService};
use actix_codec::AsyncRead;
use bytes::BytesMut;
use std::{io, pin::Pin};
use tokio::io::{AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tokio_util::codec::{Decoder, Encoder};

use crate::{router::Router, SessionManagementPacket, SessionManagementPacketResult, SessionManager, SessionState, XmppCodec};
use xmpp_proto::Packet;

pub(crate) struct TcpSession {
    _id: usize,
    _router: Addr<Router>,
    #[allow(dead_code)]
    sink: FramedWrite<Packet, Pin<Box<dyn AsyncWrite + 'static>>, XmppCodec>,
}

impl TcpSession {
    pub(crate) fn new(id: usize, router: Addr<Router>, sink: FramedWrite<Packet, Pin<Box<dyn AsyncWrite + 'static>>, XmppCodec>) -> Self {
        Self { _id: id, _router: router, sink }
    }
}

impl Actor for TcpSession {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("Starting TcpSession");
    }

    fn stopping(&mut self, _ctx: &mut Self::Context) -> actix::Running {
        println!("Stopping TcpSession");
        actix::Running::Stop
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        println!("TcpSession Stopped");
    }
}

pub trait MyTrait: AsyncRead + AsyncWrite + Unpin {}
pub struct XmppStream {
    inner: Box<dyn MyTrait>,
}

impl MyTrait for tokio_rustls::server::TlsStream<tokio::net::TcpStream> {}

impl TcpSession {
    pub async fn handle_stream(mut stream: tokio::net::TcpStream, acceptor: tokio_rustls::TlsAcceptor) -> XmppStream {
        let mut session_state: SessionState = SessionState::Opening;
        let mut buf = BytesMut::with_capacity(4096);
        let mut codec = XmppCodec::new();
        loop {
            stream.readable().await.unwrap();

            match stream.read_buf(&mut buf).await {
                Ok(0) => {}
                Ok(_) => {
                    if let Ok(Some(packet)) = codec.decode(&mut buf) {
                        match SessionManager::from_registry().send(SessionManagementPacket { session_state, packet }).await.unwrap() {
                            std::result::Result::Ok(SessionManagementPacketResult {
                                session_state: new_session_state,
                                packets,
                            }) => {
                                session_state = new_session_state;

                                println!("SessionState is {:?}", session_state);
                                packets.into_iter().for_each(|packet| {
                                    if let Err(e) = codec.encode(packet, &mut buf) {
                                        println!("Error: {:?}", e);
                                    }
                                });

                                if let Err(e) = stream.write_buf(&mut buf).await {
                                    println!("{:?}", e);
                                }

                                if let Err(e) = stream.flush().await {
                                    println!("{:?}", e);
                                }
                                match session_state {
                                    crate::SessionState::Negociating => break,
                                    _ => continue,
                                }
                            }
                            std::result::Result::Err(_) => break,
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

        let mut tls_stream = acceptor.accept(stream).await.unwrap();
        session_state = SessionState::Negociated;
        let mut buf = BytesMut::with_capacity(4096);

        loop {
            match tls_stream.read_buf(&mut buf).await {
                Ok(0) => {}
                Ok(_) => {
                    if let Ok(Some(packet)) = codec.decode(&mut buf) {
                        match SessionManager::from_registry().send(SessionManagementPacket { session_state, packet }).await.unwrap() {
                            std::result::Result::Ok(SessionManagementPacketResult {
                                session_state: new_session_state,
                                packets,
                            }) => {
                                session_state = new_session_state;

                                packets.into_iter().for_each(|packet| {
                                    println!("Let's encode a {:?}", packet);
                                    if let Err(e) = codec.encode(packet, &mut buf) {
                                        println!("Error: {:?}", e);
                                    }
                                });

                                if let Err(e) = tls_stream.write_buf(&mut buf).await {
                                    println!("Write buf error {:?}", e);
                                }

                                if let Err(e) = tls_stream.flush().await {
                                    println!("Flush error {:?}", e);
                                }
                                match session_state {
                                    crate::SessionState::Negociating => break,
                                    _ => continue,
                                }
                            }
                            std::result::Result::Err(_) => break,
                        }
                    }
                }
                Err(_) => break,
            }
        }

        XmppStream { inner: Box::new(tls_stream) }
    }
}

impl actix::io::WriteHandler<io::Error> for TcpSession {}

impl StreamHandler<Result<Packet, io::Error>> for TcpSession {
    fn handle(&mut self, _msg: Result<Packet, io::Error>, _ctx: &mut Context<Self>) {
        // match msg {
        //     Ok(Packet::NonStanza(NonStanza::OpenStream(OpenStream { to, xmlns, lang, version, from, id }))) => match self.state {
        // SessionState::Initialized => {
        //     self.sink.write(Packet::NonStanza(NonStanza::OpenStream(OpenStream {
        //         id,
        //         to: from,
        //         from: to,
        //         xmlns,
        //         lang,
        //         version,
        //     })));
        //     self.sink.write(Packet::NonStanza(NonStanza::StreamFeatures(StreamFeatures {
        //         features: Features::Mechanisms(vec!["PLAIN".to_string()]),
        //     })));
        // }
        // SessionState::Authenticated => {
        //     self.sink.write(Packet::NonStanza(NonStanza::OpenStream(OpenStream {
        //         id,
        //         to: from,
        //         from: to,
        //         xmlns,
        //         lang,
        //         version,
        //     })));

        //     self.sink.write(Packet::NonStanza(NonStanza::StreamFeatures(StreamFeatures { features: Features::Bind })));
        // }
        // },
        // Ok(Packet::NonStanza(NonStanza::ProceedTls(ProceedTls { .. }))) => {
        //     self.state = SessionState::Authenticated;
        //     self.sink.write(Packet::NonStanza(NonStanza::SASLSuccess))
        // }
        // Ok(Packet::NonStanza(NonStanza::SASLSuccess)) => self.sink.write(Packet::NonStanza(NonStanza::SASLSuccess)),
        // Ok(Packet::Stanza(Stanza::IQ(_))) => {}

        // _ => (),
        // };
    }
}
