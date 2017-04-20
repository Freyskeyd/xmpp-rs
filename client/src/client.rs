use futures::{Async, future, Future, Poll, Stream};
use futures::sync::oneshot::{self, Receiver};
use std::io::{self};
use std::sync::{Arc, Mutex};
use xmpp_proto::{XMPPConfig, ConnectionState, Connection, XMPPTransport, Credentials, XMPPCodec, XMPPStream};
use xmpp_proto::events::Event;
use xmpp_proto::events::Ping;
use native_tls::TlsConnector;
use tokio_io::AsyncRead;
use tokio_tls::TlsConnectorExt;
use tokio_core::net::TcpStream;

/// Client struct deal with connection and message sharing between server and client
#[derive(Clone)]
pub struct Client {
    transport: Arc<Mutex<XMPPTransport>>
}

impl Client {
    /// Connect a client to a server
    pub fn connect(stream: TcpStream, config: XMPPConfig, credentials: Option<Credentials>) -> Box<Future<Item=Client, Error=io::Error>>
    {
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

    /// ok
    pub fn close(&self) {
        if let Ok(mut transport) = self.transport.lock() {
            let _ = transport.shutdown();
        }
    }

    /// Send the first presence
    pub fn send_presence(&self) -> Box<Future<Item = (), Error = io::Error>> {
        if let Ok(mut transport) = self.transport.lock() {
            transport.send_presence()
                .map_err(|_| ())
                .and_then(|_| {

                    Ok(Box::new(future::ok(())))
                }).unwrap()
        } else {
            panic!("")
        }
    }

    /// Return the current Credentials
    pub fn get_jid(&self) -> Credentials {
        if let Ok(mut transport) = self.transport.lock() {
            transport.get_credentials()
        } else {
            panic!("")
        }
    }

    /// Send an event to the stream
    ///
    /// TODO: need future here
    pub fn send(&mut self, f: Event) -> Box<Future<Item = (), Error = io::Error>> {
        if let Ok(mut transport) = self.transport.lock() {
            transport.send_frame(f)
        } else {
            panic!("")
        }
    }

    /// ok
    pub fn shutdown(&mut self) -> Result<Async<()>, io::Error> {
        if let Ok(mut transport) = self.transport.lock() {
            transport.shutdown()
        } else {
            panic!("")
        }
    }

    /// Send a ping request
    pub fn send_ping(&mut self, ping: &mut Ping) -> Receiver<Event> {
        let (tx, rx) = oneshot::channel();
        if let Ok(mut transport) = self.transport.lock() {
            let jid = transport.get_credentials().jid.clone();
            let ping = ping.set_from(Some(&jid));
            transport.send_ping(tx, ping.unwrap());
        }

        rx
    }

    /// Create a consumer and return a stream that Poll every Event in the stream
    pub fn handle(&mut self) -> Box<Future<Item = Consumer, Error = io::Error>> {
        let t = self.transport.clone();
        if let Ok(mut transport) = self.transport.lock() {
            transport.send_frames();
            transport.handle_frames();
        }
        let consumer = Consumer {
            transport: t.clone()
        };

        Box::new(wait_for_answer(t.clone()).map(move |_| {
            consumer
        }))
    }
}

/// Consume message from the stream
#[derive(Clone)]
pub struct Consumer{
    /// Upstream transport readed by consumer
    pub transport: Arc<Mutex<XMPPTransport>>,
}

impl Stream for Consumer {
    type Item = Event;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Option<Event>, io::Error> {
        if let Ok(mut transport) = self.transport.try_lock() {
            if transport.connection.state == ConnectionState::Closed {
            //     return Ok(Async::Ready(None));
            }
            transport.handle_frames();
            //FIXME: if the consumer closed, we should return Ok(Async::Ready(None))
            if let Some(message) = transport.connection.next_input_frame() {
                let _ = transport.stream_poll();
                Ok(Async::Ready(Some(message)))
            } else {
                let _ = transport.stream_poll();
                Ok(Async::NotReady)
            }
        } else {
            //FIXME: return an error in case of mutex failure
            return Ok(Async::NotReady);
        }
    }
}

fn wait_for_answer(transport: Arc<Mutex<XMPPTransport>>) -> Box<Future<Item = (), Error = io::Error>> {
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
