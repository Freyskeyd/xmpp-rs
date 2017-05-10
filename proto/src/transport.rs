#![allow(unused_must_use)]
use events::Event;
use events::Ping;
use events::NonStanzaEvent::*;
use events::Event::NonStanza;
use events::EventTrait;
use std::str;
use stream::XMPPStream;
use std::io;
use futures::future;
use futures::{Async, Poll, Sink, Stream, StartSend, Future};
use connection::{ConnectionState, ConnectingState, Connection};
use credentials::Credentials;
use futures::sync::oneshot::Sender;

/// Deal with the communication process based on a Tcp/Tls stream
pub struct XMPPTransport {
    /// Current stream connected to xmpp
    pub stream: XMPPStream,
    /// Connection state machine
    pub connection: Connection,
}

impl XMPPTransport {
    /// Deal with connection
    /// TODO
    pub fn connect(upstream: XMPPStream, connection: Connection) -> Box<Future<Item = XMPPTransport, Error = io::Error>> {

        let mut t = XMPPTransport {
            stream: upstream,
            connection: connection,
        };

        match t.stream {
            XMPPStream::Tcp(ref mut stream) => {
                t.connection.connect();
                let f = t.connection.next_frame().unwrap();
                stream.start_send(f);
                stream.poll_complete();
                stream.poll();
            }
            XMPPStream::Tls(ref mut stream) => {
                t.connection.start_tls();
                let f = t.connection.next_frame().unwrap();
                stream.start_send(f);
                stream.poll_complete();
                stream.poll();
            }
        };

        let mut connector = XMPPTransportConnector { transport: Some(t) };

        connector.poll();

        Box::new(connector)
    }


    pub fn shutdown(&mut self) -> Result<Async<()>, io::Error> {
        self.stream.shutdown()
    }

    /// Send a ping IQ and register the Receiver for the response IQ
    pub fn send_ping(&mut self, tx: Sender<Event>, ping: &Ping) {
        // let ping = self.connection.compile_ping();
        let id = ping.get_id();
        let event = ping.to_event();
        // let event = Stanza(IqRequestEvent(PingIq(ping)), String::new());
        self.connection
            .iq_queue
            .insert(id.to_string(), Box::new(tx));

        match self.stream {
            XMPPStream::Tcp(ref mut stream) => {
                stream.start_send(event);
                stream.poll_complete();
                stream.poll();
            }
            XMPPStream::Tls(ref mut stream) => {
                stream.start_send(event);
                stream.poll_complete();
                stream.poll();
            }
        };
    }

    /// Send the first presence to the stream
    pub fn send_presence(&mut self) -> Result<Async<Option<Event>>, io::Error> {
        self.connection.compile_presence();

        match self.stream {
            XMPPStream::Tcp(ref mut stream) => {
                let f = self.connection.next_frame().unwrap();
                stream.start_send(f);
                stream.poll_complete();
                stream.poll()
            }
            XMPPStream::Tls(ref mut stream) => {
                let f = self.connection.next_frame().unwrap();
                stream.start_send(f);
                stream.poll_complete();
                stream.poll()
            }
        }
    }

    /// Return the current credentials
    ///
    /// TODO
    pub fn get_credentials(&mut self) -> Credentials {
        match self.connection.credentials {
            Some(ref c) => c.clone(),
            None => panic!(""),
        }
    }

    /// Execute poll on the stream
    ///
    /// DRY
    pub fn stream_poll(&mut self) -> Result<Async<Option<Event>>, io::Error> {
        match self.stream {
            XMPPStream::Tcp(ref mut s) => s.poll(),
            XMPPStream::Tls(ref mut s) => s.poll(),
        }
    }

    /// Send pendings Events to the stream
    pub fn send_frames(&mut self) {
        //FIXME: find a way to use a future here
        while let Some(f) = self.connection.next_frame() {
            match self.stream {
                XMPPStream::Tls(ref mut stream) => {
                    stream.start_send(f);
                    stream.poll_complete();
                }
                XMPPStream::Tcp(_) => panic!(""),
            }
        }
    }

    /// Send an Event to the stream
    pub fn send_frame(&mut self, s: Event) -> Box<Future<Item = (), Error = io::Error>> {
        Box::new(match self.stream {
                     XMPPStream::Tls(ref mut stream) => {
                         stream.start_send(s);
                         stream.poll_complete();
                         future::ok(())
                     }
                     XMPPStream::Tcp(_) => panic!(""),
                 })
    }

    /// TODO@
    pub fn handle_frames(&mut self) {
        loop {
            match self.poll() {
                Ok(Async::Ready(Some(frame))) => {
                    trace!("handle frames: AMQPTransport received frame: {:?}", frame);
                    self.connection.handle_frame(frame);
                }
                Ok(Async::Ready(None)) => {
                    trace!("handle frames: upstream poll gave Ready(None)");
                    break;
                }
                Ok(Async::NotReady) => {
                    trace!("handle frames: upstream poll gave NotReady");
                    match self.stream {
                        XMPPStream::Tls(ref mut stream) => {
                            stream.poll();
                            stream.poll_complete();
                        }
                        XMPPStream::Tcp(_) => panic!(""),
                    }
                    break;
                }
                Err(e) => {
                    error!("handle frames: upstream poll got error: {:?}", e);
                    break;
                }
            };
        }
    }
}

/// Struct use in the connection process
struct XMPPTransportConnector {
    /// Upstream transport used to etablish connection
    pub transport: Option<XMPPTransport>,
}

impl Future for XMPPTransportConnector {
    type Item = XMPPTransport;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        trace!("XMPPTransportConnector poll transport is none? {}",
               self.transport.is_none());
        let mut transport = self.transport.take().unwrap();
        trace!("conn state: {:?}", transport.connection.state);
        if transport.connection.state == ConnectionState::Connected {
            trace!("already connected");
            return Ok(Async::Ready(transport));
        }

        while let Some(f) = match match transport.stream {
                        XMPPStream::Tcp(ref mut stream) => stream.poll(),
                        XMPPStream::Tls(ref mut stream) => stream.poll(),
                    } {
                  Ok(Async::Ready(t)) => t,
                  Ok(Async::NotReady) => {
                      trace!("stream poll gave NotReady");
                      match transport.stream {
                          XMPPStream::Tcp(ref mut stream) => stream.poll(),
                          XMPPStream::Tls(ref mut stream) => stream.poll(),
                      };
                      self.transport = Some(transport);
                      return Ok(Async::NotReady);
                  }
                  Err(e) => {
                      error!("stream poll got error: {:?}", e);
                      return Err(From::from(e));
                  }
              } {
            transport.connection.handle_frame(f);
            while let Some(fr) = transport.connection.next_frame() {
                if let NonStanza(ref non_stanza) = fr {
                    match **non_stanza {
                        StartTlsEvent(_) => transport.connection.state = ConnectionState::Connecting(ConnectingState::SentAuthenticationMechanism),
                        OpenStreamEvent(_) => {
                            if transport.connection.state == ConnectionState::Connecting(ConnectingState::ReceivedProceedCommand) {
                                return Ok(Async::Ready(transport));
                            }
                        }
                        _ => {}
                    }
                }

                match transport.stream {
                    XMPPStream::Tcp(ref mut stream) => {
                        stream.start_send(fr);
                        stream.poll_complete()
                    }
                    XMPPStream::Tls(ref mut stream) => {
                        stream.start_send(fr);
                        stream.poll_complete()
                    }
                };
            }
            if transport.connection.state == ConnectionState::Connected {
                return Ok(Async::Ready(transport));
            }
        }

        self.transport = Some(transport);
        Ok(Async::NotReady)
    }
}

impl Stream for XMPPTransport {
    type Item = Event;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, io::Error> {
        // trace!("Connected:: Polling connected stream");
        while let Some(frame) = match match self.stream {
                        XMPPStream::Tcp(ref mut stream) => stream.poll(),
                        XMPPStream::Tls(ref mut stream) => stream.poll(),
                    } {
                  Ok(Async::Ready(t)) => t,
                  Ok(Async::NotReady) => {
                      trace!("Connected:: stream poll gave NotReady");
                      match self.stream {
                          XMPPStream::Tcp(ref mut stream) => stream.poll(),
                          XMPPStream::Tls(ref mut stream) => stream.poll(),
                      };
                      // self.transport = Some(transport);
                      return Ok(Async::NotReady);
                  }
                  Err(e) => {
                      error!("Connected:: stream poll got error: {:?}", e);
                      return Err(From::from(e));
                  }
              } {
            self.connection.handle_frame(frame);
        }

        Ok(Async::NotReady)
    }
}

impl Sink for XMPPTransport {
    type SinkItem = Event;
    type SinkError = io::Error;

    fn start_send(&mut self, item: Event) -> StartSend<Event, io::Error> {
        trace!("Start send this item: {:?}", item);
        match self.stream {
            XMPPStream::Tcp(ref mut stream) => stream.start_send(item),
            XMPPStream::Tls(ref mut stream) => stream.start_send(item),
        }
    }

    fn poll_complete(&mut self) -> Poll<(), io::Error> {
        match self.stream {
            XMPPStream::Tcp(ref mut stream) => stream.poll_complete(),
            XMPPStream::Tls(ref mut stream) => stream.poll_complete(),
        }
    }
}
