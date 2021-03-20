use tokio_core::net::TcpStream;
use tokio_tls::TlsStream;
use tokio_io::codec::Framed;
use codec::XMPPCodec;
use std::io;
use futures::Async;
// use futures::Sink;
use tokio_io::AsyncWrite;
// use std::net::Shutdown;

/// Can be both Tcp or Tls. Own the TcpStream of a connection.
pub enum XMPPStream {
    Tcp(Framed<TcpStream, XMPPCodec>),
    Tls(Framed<TlsStream<TcpStream>, XMPPCodec>),
}

impl XMPPStream {
    pub fn check_connectivity(&mut self) -> Async<()> {
        match *self {
            XMPPStream::Tcp(ref mut stream) => stream.get_mut().poll_read(),
            XMPPStream::Tls(ref mut stream) => stream.get_mut().get_mut().get_mut().poll_read(),
        }
    }

    pub fn shutdown(&mut self) -> Result<Async<()>, io::Error> {
        match *self {
            XMPPStream::Tcp(ref mut stream) => stream.get_mut().shutdown(),
            XMPPStream::Tls(ref mut stream) => stream.get_mut().shutdown(),
        }
    }
}
