#![allow(unused_must_use)]

use stream::XMPPStream;
use std::io;
use futures::future;
use futures::{Async,Poll,Sink,Stream,StartSend,Future};
use connection::{ConnectionState, ConnectingState, Connection};

pub struct XMPPTransport {
    pub stream: XMPPStream,
    pub connection: Connection
}

impl XMPPTransport
{
    pub fn connect(upstream: XMPPStream, connection: Connection) -> Box<Future<Item=XMPPTransport, Error=io::Error>>
    {

        let mut t = XMPPTransport {
            stream: upstream,
            connection: connection
        };

        match t.stream {
            XMPPStream::Tcp(ref mut stream) => {
                t.connection.connect();
                let f = t.connection.next_frame().unwrap();
                stream.start_send(f);
                stream.poll_complete();
                stream.poll();
            },
            XMPPStream::Tls(ref mut stream) => {
                t.connection.start_tls();
                let f = t.connection.next_frame().unwrap();
                stream.start_send(f);
                stream.poll_complete();
                stream.poll();
            }
        };

        let mut connector = XMPPTransportConnector {
            transport: Some(t)
        };

        connector.poll();

        Box::new(connector)
    }

    pub fn stream_poll(&mut self) {
        match self.stream {
            XMPPStream::Tcp(ref mut s) => s.poll(),
            XMPPStream::Tls(ref mut s) => s.poll()
        };
    }

    pub fn send_frames(&mut self) {
        //FIXME: find a way to use a future here
        while let Some(f) = self.connection.next_frame() {
            match self.stream {
                XMPPStream::Tls(ref mut stream) => {
                    stream.start_send(f);
                    stream.poll_complete();
                },
                XMPPStream::Tcp(_) => panic!("")
            }
        }
        //self.upstream.poll_complete();
    }

    pub fn send_frame(&mut self, s: String) -> Box<Future<Item = (), Error = io::Error>> {
        Box::new(match self.stream {
            XMPPStream::Tls(ref mut stream) => {
                stream.start_send(s);
                stream.poll_complete();
                future::ok(())
            },
            XMPPStream::Tcp(_) => panic!("")
        })
    }

    pub fn handle_frames(&mut self) {
        loop {
            match self.poll() {
                Ok(Async::Ready(Some(frame))) => {
                    trace!("handle frames: AMQPTransport received frame: {:?}", frame);
                    self.connection.handle_frame(frame);
                },
                Ok(Async::Ready(None)) => {
                    trace!("handle frames: upstream poll gave Ready(None)");
                    break;
                },
                Ok(Async::NotReady) => {
                    trace!("handle frames: upstream poll gave NotReady");
                    match self.stream {
                        XMPPStream::Tls(ref mut stream) => {
                            stream.poll();
                        },
                        XMPPStream::Tcp(_) => panic!("")
                    }
                    break;
                },
                Err(e) => {
                    error!("handle frames: upstream poll got error: {:?}", e);
                    break;
                },
            };
        }
    }
}


pub struct XMPPTransportConnector {
    pub transport: Option<XMPPTransport>
}

impl Future for XMPPTransportConnector
{

    type Item  = XMPPTransport;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        trace!("XMPPTransportConnector poll transport is none? {}", self.transport.is_none());
        let mut transport = self.transport.take().unwrap();
        trace!("conn state: {:?}", transport.connection.state);
        if transport.connection.state == ConnectionState::Connected {
            trace!("already connected");
            return Ok(Async::Ready(transport))
        }

        trace!("waiting before poll");
        let value = match match transport.stream {
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
            },
            Err(e) => {
                error!("stream poll got error: {:?}", e);
                return Err(From::from(e));
            },
        };

        match value {
            Some(frame) => {
                println!("got frame: {:?}", frame);
                transport.connection.handle_frame(frame);
                while let Some(f) = transport.connection.next_frame() {
                    if f.contains("starttls") {
                        transport.connection.state = ConnectionState::Connecting(ConnectingState::SentAuthenticationMechanism)
                    }
                    if f.contains("stream:stream") && transport.connection.state == ConnectionState::Connecting(ConnectingState::ReceivedProceedCommand) {
                        return Ok(Async::Ready(transport));
                    }

                    match transport.stream {
                        XMPPStream::Tcp(ref mut stream) => {
                            stream.start_send(f);
                            stream.poll_complete()
                        },
                        XMPPStream::Tls(ref mut stream) => {
                            stream.start_send(f);
                            stream.poll_complete()
                        }
                    };

                }

                match transport.stream {
                    XMPPStream::Tcp(ref mut stream) => stream.poll_complete(),
                    XMPPStream::Tls(ref mut stream) => stream.poll_complete(),
                };
                if transport.connection.state == ConnectionState::Connected {
                    return Ok(Async::Ready(transport))
                } else {
                    trace!("Not Ready!");
                    match transport.stream {
                        XMPPStream::Tcp(ref mut stream) => stream.poll(),
                        XMPPStream::Tls(ref mut stream) => stream.poll(),
                    };
                    self.transport = Some(transport);
                    return Ok(Async::NotReady)
                }
            },
            e => {
                error!("did not get a frame? -> {:?}", e);

                self.transport = Some(transport);
                return Ok(Async::NotReady)
            }
        }
    }
}

impl Stream for XMPPTransport
// where T: AsyncRead
{
    type Item = String;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Option<String>, io::Error> {
        match try_ready!(match self.stream {
            XMPPStream::Tcp(ref mut stream) => stream.poll(),
            XMPPStream::Tls(ref mut stream) => stream.poll(),
        }) {
            Some(frame) => {
                trace!("XMPPTransport received frame: {:?}", frame);
                try!(self.poll_complete());
                return Ok(Async::Ready(Some(frame)))
            },
            None => {
                trace!("XMPPTransport returned NotReady");
                return Ok(Async::NotReady)
            }
        }
    }
}

impl Sink for XMPPTransport
// where T: AsyncWrite
{
    type SinkItem = String;
    type SinkError = io::Error;

    fn start_send(&mut self, item: String) -> StartSend<String, io::Error> {
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
