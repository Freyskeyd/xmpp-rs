use crate::messages::tcp::TcpOpenStream;
use crate::{
    listeners::XmppStream,
    messages::SessionManagementPacketResult,
    packet::PacketHandler,
    parser::codec::XmppCodec,
    sessions::{state::SessionState, unauthenticated::UnauthenticatedSession},
};
use actix::prelude::*;
use actix_codec::{Decoder, Encoder};
use bytes::BytesMut;
use log::{error, trace};
use std::io;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    sync::mpsc::{self, Receiver, Sender},
};

pub(crate) mod listener;
pub(crate) mod session;

impl Handler<TcpOpenStream> for UnauthenticatedSession {
    type Result = ResponseFuture<Result<XmppStream, ()>>;

    fn handle(&mut self, msg: TcpOpenStream, _ctx: &mut Self::Context) -> Self::Result {
        trace!("Opening TCP");
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
                        while let Ok(Some(packet)) = codec.decode(&mut buf) {
                            let result = Self::handle_packet(&state, &packet, Some(tx.clone())).await;

                            if result.is_ok() {
                                trace!("Waiting for response");

                                if let Some(SessionManagementPacketResult { session_state, packets }) = rx.recv().await {
                                    state = session_state;

                                    trace!("SessionState is {:?}", session_state);

                                    packets.into_iter().for_each(|packet| {
                                        if let Err(e) = codec.encode(packet, &mut buf) {
                                            error!("Error in proceed_packet: {:?}", e);
                                        }
                                    });

                                    if let Err(e) = stream.write_buf(&mut buf).await {
                                        error!("{:?}", e);
                                    }

                                    if let Err(e) = stream.flush().await {
                                        error!("{:?}", e);
                                    }
                                }
                            }
                        }
                        match state {
                            SessionState::Negociating => break,
                            SessionState::Closing => {
                                // TODO: remove unwrap
                                let _ = stream.into_std().unwrap().shutdown(std::net::Shutdown::Both);
                                return Err(());
                            }
                            _ => {}
                        }
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                        continue;
                    }
                    Err(e) => {
                        error!("err: {:?}", e);
                        break;
                    }
                }
            }

            match acceptor {
                Some(acceptor) => {
                    trace!("Session switching to TLS");
                    let mut tls_stream = acceptor.accept(stream).await.unwrap();
                    state = SessionState::Negociated;
                    let mut buf = BytesMut::with_capacity(4096);

                    loop {
                        match tls_stream.read_buf(&mut buf).await {
                            Ok(0) => {}
                            Ok(_) => {
                                while let Ok(Some(packet)) = codec.decode(&mut buf) {
                                    let result = Self::handle_packet(&state, &packet, Some(tx.clone())).await;

                                    println!("{:?}", result);
                                    if result.is_ok() {
                                        trace!("Waiting for response");

                                        if let Some(SessionManagementPacketResult { session_state, packets }) = rx.recv().await {
                                            state = session_state;

                                            trace!("SessionState is {:?}", session_state);

                                            packets.into_iter().for_each(|packet| {
                                                if let Err(e) = codec.encode(packet, &mut buf) {
                                                    error!("Error in proceed_packet: {:?}", e);
                                                }
                                            });

                                            if let Err(e) = tls_stream.write_buf(&mut buf).await {
                                                error!("{:?}", e);
                                            }

                                            if let Err(e) = tls_stream.flush().await {
                                                error!("{:?}", e);
                                            }
                                        }
                                    }
                                }

                                if state == SessionState::Authenticated {
                                    break;
                                }
                                if state == SessionState::Closing {
                                    // TODO: remove unwrap
                                    let (inner_stream, _) = tls_stream.into_inner();
                                    let _ = inner_stream.into_std().unwrap().shutdown(std::net::Shutdown::Both);
                                    return Err(());
                                }
                            }
                            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                                continue;
                            }
                            Err(e) => {
                                // return Err(e.into());
                                error!("err: {:?}", e);
                                break;
                            }
                        };
                    }

                    Ok(XmppStream { inner: Box::new(tls_stream) })
                }
                None => Ok(XmppStream { inner: Box::new(stream) }),
            }
        };
        Box::pin(fut)
    }
}
