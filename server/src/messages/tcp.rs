use crate::listeners::XmppStreamHolder;
use crate::router::Router;
use actix::{prelude::*, Message};
use tokio::net::TcpStream;

#[derive(Message)]
#[rtype("()")]
pub struct NewSession(pub TcpStream, pub std::net::SocketAddr, pub Addr<Router>);

#[derive(Message)]
#[rtype("Result<XmppStreamHolder, ()>")]
pub struct TcpOpenStream {
    pub(crate) stream: TcpStream,
    pub(crate) acceptor: Option<tokio_rustls::TlsAcceptor>,
}
