use crate::router::Router;
use crate::{
    listeners::XmppStream,
    sessions::{manager::SessionManager, state::SessionState, unauthenticated::UnauthenticatedSession, SessionManagementPacket, SessionManagementPacketResult},
    XmppCodec,
};
use actix::{prelude::*, Message};
use actix_codec::{Decoder, Encoder};
use bytes::BytesMut;
use std::io;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::mpsc::{self, Receiver, Sender},
};

pub(crate) mod listener;
pub(crate) mod session;

#[derive(Message)]
#[rtype("()")]
pub struct NewSession(pub TcpStream, pub std::net::SocketAddr, pub Addr<Router>);

#[derive(Message)]
#[rtype("Result<XmppStream, ()>")]
pub struct TcpOpenStream {
    stream: TcpStream,
    acceptor: tokio_rustls::TlsAcceptor,
}

impl Handler<TcpOpenStream> for UnauthenticatedSession {
    type Result = ResponseFuture<Result<XmppStream, ()>>;

    fn handle(&mut self, msg: TcpOpenStream, _ctx: &mut Self::Context) -> Self::Result {
        println!("Opening TCP");
        let mut stream = msg.stream;
        let acceptor = msg.acceptor;
        let mut state = self.state;
        let mut buf = BytesMut::with_capacity(4096);
        let mut codec = XmppCodec::new();

        let fut = async move {
            let (tx, mut rx): (Sender<SessionManagementPacketResult>, Receiver<SessionManagementPacketResult>) = mpsc::channel(32);
            loop {
                stream.readable().await.unwrap();

                match stream.read_buf(&mut buf).await {
                    Ok(0) => {}
                    Ok(_) => {
                        if let Ok(Some(packet)) = codec.decode(&mut buf) {
                            println!("Sending packet to manager");
                            SessionManager::from_registry().do_send(SessionManagementPacket {
                                session_state: state,
                                packet,
                                referer: tx.clone(),
                            });
                        }
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                        continue;
                    }
                    Err(e) => {
                        println!("err: {:?}", e);
                        break;
                    }
                }

                if let Some(SessionManagementPacketResult { session_state, packets }) = rx.recv().await {
                    state = session_state;

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
                        SessionState::Negociating => break,
                        _ => continue,
                    }
                }
            }

            println!("Session switching to TLS");
            let mut tls_stream = acceptor.accept(stream).await.unwrap();
            state = SessionState::Negociated;
            let mut buf = BytesMut::with_capacity(4096);

            loop {
                match tls_stream.read_buf(&mut buf).await {
                    Ok(0) => {}
                    Ok(_) => {
                        if let Ok(Some(packet)) = codec.decode(&mut buf) {
                            println!("Sending packet to manager");
                            SessionManager::from_registry().do_send(SessionManagementPacket {
                                session_state: state,
                                packet,
                                referer: tx.clone(),
                            });
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

                if let Some(SessionManagementPacketResult { session_state, packets }) = rx.recv().await {
                    state = session_state;

                    println!("SessionState is {:?}", session_state);

                    packets.into_iter().for_each(|packet| {
                        if let Err(e) = codec.encode(packet, &mut buf) {
                            println!("Error: {:?}", e);
                        }
                    });

                    if let Err(e) = tls_stream.write_buf(&mut buf).await {
                        println!("{:?}", e);
                    }

                    if let Err(e) = tls_stream.flush().await {
                        println!("{:?}", e);
                    }
                    match session_state {
                        SessionState::Negociating => break,
                        _ => continue,
                    }
                }
            }

            Ok(XmppStream { inner: Box::new(tls_stream) })
        };
        Box::pin(fut)
    }
}
