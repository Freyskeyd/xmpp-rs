use futures::{Async,Poll};
use std::sync::Arc;
use std::sync::Mutex;
use tokio_io::{AsyncRead};
use std::io::{self};
use futures::Future;
use futures::future;
use xmpp_proto::transport::XMPPTransport;
use xmpp_proto::codec::XMPPCodec;
use tokio_core::net::TcpStream;
use tokio_tls::TlsConnectorExt;
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
        let config = XMPPConfig::new();
        let connection = Connection::new(config);
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

    pub fn send_presence(&self) -> Box<Future<Item = (), Error = io::Error>> {
        if let Ok(mut transport) = self.transport.lock() {
            transport.connection.send_presence();
            transport.send_frames();
            transport.handle_frames();

            Box::new(future::ok(()))
        } else {
            panic!("")
        }
    }
    pub fn send(&mut self, f: String) -> Box<Future<Item = (), Error = io::Error>> {
        if let Ok(mut transport) = self.transport.lock() {
            transport.send_frame(f)
        } else {
            panic!("")
        }
    }

    pub fn handle(&mut self) -> Box<Future<Item = Consumer, Error = io::Error>> {
        let t = self.transport.clone();
        if let Ok(mut transport) = self.transport.lock() {
            transport.send_frames();
            transport.handle_frames();

            let consumer = Consumer {
                transport: t.clone()
            };

            Box::new(wait_for_answer(t.clone()).map(move |_| {
                consumer
            }))
        } else {
            panic!("")
        }
    }
}

#[derive(Clone)]
pub struct Consumer{
  pub transport: Arc<Mutex<XMPPTransport>>,
}

impl Stream for Consumer {
  type Item = String;
  type Error = io::Error;

  fn poll(&mut self) -> Poll<Option<String>, io::Error> {
    //trace!("consumer[{}] poll", self.consumer_tag);
    if let Ok(mut transport) = self.transport.try_lock() {
      transport.handle_frames();
      //FIXME: if the consumer closed, we should return Ok(Async::Ready(None))
      if let Some(message) = transport.connection.next_input_frame() {
          transport.stream_poll();
          Ok(Async::Ready(Some(message)))
      } else {
          transport.stream_poll();
          Ok(Async::NotReady)
      }
    } else {
      //FIXME: return an error in case of mutex failure
      return Ok(Async::NotReady);
    }
  }
}

pub fn wait_for_answer(transport: Arc<Mutex<XMPPTransport>>) -> Box<Future<Item = (), Error = io::Error>> {
  Box::new(future::poll_fn(move || {
    let connected = if let Ok(mut tr) = transport.try_lock() {
      tr.handle_frames();
      true
    } else {
      return Ok(Async::NotReady);
    };

    if connected {
      Ok(Async::Ready(()))
    } else {
      Ok(Async::NotReady)
    }
  }))

}
