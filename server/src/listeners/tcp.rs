use crate::router::Router;
use crate::{
    listeners::XmppStream,
    parser::codec::XmppCodec,
    sessions::{state::SessionState, unauthenticated::UnauthenticatedSession, SessionManagementPacketResult},
};
use actix::{prelude::*, Message};
use actix_codec::Decoder;
use bytes::BytesMut;
use log::{error, trace};
use std::io;
use tokio::{
    io::AsyncReadExt,
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
                            match Self::proceed_packet(packet, state, tx.clone(), &mut rx, &mut codec, &mut stream, &mut buf).await {
                                Ok(new_state) => state = new_state,
                                Err(_) => break,
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

            trace!("Session switching to TLS");
            let mut tls_stream = acceptor.accept(stream).await.unwrap();
            state = SessionState::Negociated;
            let mut buf = BytesMut::with_capacity(4096);

            loop {
                match tls_stream.read_buf(&mut buf).await {
                    Ok(0) => {}
                    Ok(_) => {
                        while let Ok(Some(packet)) = codec.decode(&mut buf) {
                            match Self::proceed_packet(packet, state, tx.clone(), &mut rx, &mut codec, &mut tls_stream, &mut buf).await {
                                Ok(new_state) => state = new_state,
                                Err(_) => break,
                            }
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
        };
        Box::pin(fut)
    }
}
