use tokio_core::net::TcpStream;
use tokio_tls::TlsStream;
use tokio_io::codec::Framed;
use codec::XMPPCodec;

pub enum XMPPStream {
    Tcp(Framed<TcpStream, XMPPCodec>),
    Tls(Framed<TlsStream<TcpStream>, XMPPCodec>)
}
