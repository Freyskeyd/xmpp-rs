//! A simple client and server implementation fo a multiplexed, line-based
//! protocol

// #![deny(warnings, missing_docs)]

extern crate futures;
extern crate tokio_core;
extern crate tokio_proto;
extern crate tokio_service;
extern crate byteorder;

// extern crate native_tls;
// extern crate tokio_tls;

// use std::net::ToSocketAddrs;

// use native_tls::TlsConnector;
// use tokio_core::reactor::Core;
// use tokio_tls::TlsConnectorExt;

use futures::{future, Future};
use tokio_core::io::{Io, Codec, EasyBuf, Framed};
use tokio_core::net::TcpStream;
use tokio_core::reactor::Handle;
use tokio_proto::{TcpClient};
use tokio_service::{Service};
use std::{io, str};
use std::net::SocketAddr;

use tokio_proto::pipeline::{ClientProto, ClientService};

/// Multiplexed xmpp-based client handle
///
/// This type just wraps the inner service. This is done to encapsulate the
/// details of how the inner service is structured. Specifically, we don't want
/// the type signature of our client to be:
///
///   Validate<ClientService<TcpStream, XmppProto>>
///
/// This also allows adding higher level API functions that are protocol
/// specific. For example, our xmpp client has a `ping()` function, which sends
/// a "ping" request.
pub struct XmppClient {
    inner: Validate<ClientService<TcpStream, XmppProto>>,
}

/// A `Service` middleware that validates the correctness of requests and
/// responses.
struct Validate<T> {
    inner: T,
}

/// Our multiplexed xmpp-based codec
struct XmppCodec;

/// Protocol definition
struct XmppProto;

impl XmppClient {
    /// Establish a connection to a multiplexed line-based server at the
    /// provided `addr`.
    pub fn connect(addr: &SocketAddr, handle: &Handle) -> Box<Future<Item = XmppClient, Error = io::Error>> {
        let ret = TcpClient::new(XmppProto)
            .connect(addr, handle)
            .map(|client_service| {
                let validate = Validate { inner: client_service};
                XmppClient { inner: validate }
            })
        .and_then(|xmpp_client| {
            xmpp_client.start().map(|x| {
                println!("{:?}", x);
                xmpp_client
            })
        });

        Box::new(ret)
    }
    /// OK
    pub fn handle(&self) {

    }
}

/// Some doc
pub trait XmppService : Service {
    /// Some doc
    fn start(&self) -> <Self as Service>::Future;
}

impl Service for XmppClient {
    type Request = String;
    type Response = String;
    type Error = io::Error;

    // For simplicity, box the future.
    type Future = Box<Future<Item = String, Error = io::Error>>;

    fn call(&self, req: String) -> Self::Future {
        self.inner.call(req)
    }
}
impl XmppService for XmppClient {
    fn start(&self) -> <Self as Service>::Future {
        self.call(format!("<?xml version='1.0'?><stream:stream xmlns:stream='http://etherx.jabber.org/streams' xmlns='jabber:client' version='1.0' to='{}'>", "example.com"))
    }
}

impl<T> Service for Validate<T>
    where T: Service<Request = String, Response = String, Error = io::Error>,
          T::Future: 'static,
{
    type Request = String;
    type Response = String;
    type Error = io::Error;
    // For simplicity, box the future.
    type Future = Box<Future<Item = String, Error = io::Error>>;

    fn call(&self, req: String) -> Self::Future {
        // Make sure that the request does not include any new lines
        if req.chars().find(|&c| c == '\n').is_some() {
            let err = io::Error::new(io::ErrorKind::InvalidInput, "message contained new line");
            return Box::new(future::done(Err(err)))
        }

        // Call the upstream service and validate the response
        Box::new(self.inner.call(req)
            .and_then(|resp| {
                if resp.chars().find(|&c| c == '\n').is_some() {
                    Err(io::Error::new(io::ErrorKind::InvalidInput, "message contained new line"))
                } else {
                    Ok(resp)
                }
            }))
    }
}

/// Implementation of the multiplexed line-based protocol.
///
/// Frames begin with a 4 byte header, consisting of the numeric request ID
/// encoded in network order, followed by the frame payload encoded as a UTF-8
/// string and terminated with a '\n' character:
///
/// # An example frame:
///
/// +-- request id --+------- frame payload --------+
/// |                |                              |
/// |   \x00000001   | This is the frame payload \n |
/// |                |                              |
/// +----------------+------------------------------+
///
impl Codec for XmppCodec {
    type In = (String);
    type Out = (String);

    fn decode(&mut self, buf: &mut EasyBuf) -> Result<Option<(String)>, io::Error> {
        // At least 5 bytes are required for a frame: 4 byte head + one byte
        // '\n'
        // println!("IN: {:?}", str::from_utf8(buf.as_slice()));
        if buf.len() < 5 {
            return Ok(None);
        }

        // Check to see if the frame contains a new line, skipping the first 4
        // bytes which is the request ID
        if let Some(n) = buf.as_ref().iter().position(|b| *b == b'>') {
            let line = buf.drain_to(n+1);
            return match str::from_utf8(&line.as_ref()) {
                Ok(s) => {println!("{}", s);Ok(Some(s.to_string()))},
                Err(_) => Err(io::Error::new(io::ErrorKind::Other, "invalid string")),
            }
            // remove the serialized frame from the buffer.

            // // Also remove the '\n'
            // buf.drain_to(1);

            // // Turn this data into a UTF string and return it in a Frame.
            // return match str::from_utf8(&line.as_ref()) {
            //     Ok(s) => Ok(Some(s.to_string())),
            //     Err(_) => Err(io::Error::new(io::ErrorKind::Other, "invalid string")),
            // }
        }

        Ok(None)
    }

    fn encode(&mut self, msg: String, buf: &mut Vec<u8>) -> io::Result<()> {
        let msg = msg;

        buf.extend(msg.as_bytes());

        // println!("OUT: {:?}", str::from_utf8(buf.as_slice()));
        Ok(())
    }
}

impl<T: Io + 'static> ClientProto<T> for XmppProto {
    type Request = String;
    type Response = String;

    /// `Framed<T, LineCodec>` is the return value of `io.framed(LineCodec)`
    type Transport = Framed<T, XmppCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(XmppCodec))
    }
}
