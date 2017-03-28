use futures::{Async,Poll,Sink,StartSend};
use std::sync::Arc;
use std::sync::Mutex;
use tokio_io::{AsyncRead,AsyncWrite};
use std::io::{self,Error,ErrorKind};
use futures::Future;
use futures::future;
use xmpp_proto::transport::XMPPTransport;
use xmpp_proto::codec::XMPPCodec;
use tokio_core::net::TcpStream;
use tokio_tls::TlsConnectorExt;
use tokio_tls::TlsStream;
use native_tls::TlsConnector;
use xmpp_proto::connection::Connection;
use xmpp_proto::stream::XMPPStream;
use xmpp_proto::config::XMPPConfig;
use futures::Stream;

#[derive(Clone)]
pub struct Client {
    transport: Arc<Mutex<XMPPTransport>>,
}
impl Client {
    pub fn connect(stream: TcpStream) -> Box<Future<Item=Client, Error=io::Error>> {
        let mut config = XMPPConfig::new();
        let mut connection = Connection::new(config);
        Box::new(XMPPTransport::connect(XMPPStream::Tcp(stream.framed(XMPPCodec)), connection)
                 .and_then(|transport| {
                     let builder = TlsConnector::builder().unwrap();
                     let cx = builder.build().unwrap();

                     let connection = transport.connection;
                     let stream = match transport.stream {
                         XMPPStream::Tcp(stream) => stream.into_inner(),
                         XMPPStream::Tls(_) => panic!("")
                     };

                     cx.connect_async("127.0.0.1", stream).map_err(|e| {
                         io::Error::new(io::ErrorKind::Other, e)
                     }).map(|socket| (connection, socket))
                 })
                 .and_then(|(connection, s)| {
                     XMPPTransport::connect(XMPPStream::Tls(s.framed(XMPPCodec)), connection)
                 }).and_then(|transport| {

                     let client = Client {
                         transport: Arc::new(Mutex::new(transport)),
                     };

                     future::ok(client)
                 }))
    }

    pub fn send_ping(&mut self) /*-> Box<Future<Item = String, Error = io::Error>> */{
        if let Ok(mut transport) = self.transport.lock() {
            transport.connection.add_frame("<iq from='admin@bot.simon.iadvize.com' to='bot.simon.iadvize.com' id='c2s1' type='get'><ping xmlns='urn:xmpp:ping'/></iq>".to_string());
            transport.send_frames();
            transport.handle_frames();
        }

    }
}
