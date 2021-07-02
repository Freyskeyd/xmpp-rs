// TCP
pub(crate) mod tcp;
// WS
pub(crate) mod ws;

use actix_codec::AsyncRead;
use tokio::io::AsyncWrite;

// TODO: Rename this trait
pub trait MyTrait: AsyncRead + AsyncWrite + Unpin + Send {}
pub struct XmppStream {
    inner: Box<dyn MyTrait>,
}

impl MyTrait for tokio::net::TcpStream {}
impl MyTrait for tokio_rustls::server::TlsStream<tokio::net::TcpStream> {}
