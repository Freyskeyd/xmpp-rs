#![allow(unused_must_use)]
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
use xmpp_proto::credentials::Credentials;
use xmpp_proto::events::Event;

#[derive(Clone)]
pub struct Client {
  transport: Arc<Mutex<XMPPTransport>>,
}
impl Client {
  pub fn connect(stream: TcpStream, config: XMPPConfig, credentials: Option<Credentials>) -> Box<Future<Item=Client, Error=io::Error>> {
    let connection = Connection::new(&config, credentials);
    Box::new(XMPPTransport::connect(XMPPStream::Tcp(stream.framed(XMPPCodec)), connection)
             .and_then(move |transport| {
               let builder = TlsConnector::builder().unwrap();
               let cx = builder.build().unwrap();

               let connection = transport.connection;
               let stream = match transport.stream {
                 XMPPStream::Tcp(stream) => stream.into_inner(),
                 XMPPStream::Tls(_) => panic!("")
               };

               cx.connect_async(config.get_domain(), stream).map_err(|e| {
                 io::Error::new(io::ErrorKind::Other, e)
               }).map(|socket| (connection, socket))
             })
             .and_then(|(connection, s)| {
               XMPPTransport::connect(XMPPStream::Tls(s.framed(XMPPCodec)), connection)
             }).and_then(|transport| {

               let client = Client {
                 transport: Arc::new(Mutex::new(transport)),
               };

               if let Ok(mut transport) = client.transport.lock() {
                 transport.handle_frames();
               }

               future::ok(client)
             }))
  }

  pub fn send_ping(&self) -> Box<Future<Item = (), Error = io::Error>> {
    if let Ok(mut transport) = self.transport.lock() {
      transport.send_ping()
        .and_then(|_| {
          Ok(Box::new(future::ok(())))
        }).unwrap()
    } else {
      panic!("")
    }
  }

  pub fn send_presence(&self) -> Box<Future<Item = (), Error = io::Error>> {
    if let Ok(mut transport) = self.transport.lock() {
      transport.send_presence()
        .and_then(|_| {

          Ok(Box::new(future::ok(())))
        }).unwrap()
    } else {
      panic!("")
    }
  }

  pub fn get_jid(&self) -> Credentials {
    if let Ok(mut transport) = self.transport.lock() {
      transport.get_credentials()
    } else {
      panic!("")
    }
  }
  pub fn send(&mut self, f: Event) -> Box<Future<Item = (), Error = io::Error>> {
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
  type Item = Event;
  type Error = io::Error;

  fn poll(&mut self) -> Poll<Option<Event>, io::Error> {
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
