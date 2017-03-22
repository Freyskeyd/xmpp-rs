use bytes::{BytesMut};
use futures::{Future, Stream, Sink};
use native_tls::TlsConnector;
use std::str;
use std::{io};
use tokio_core::net::TcpStream as TokioStream;
use std::net::TcpStream;
use tokio_core::reactor::Core;
use tokio_io::codec::{Encoder, Decoder};
use tokio_io::{AsyncRead};
use tokio_tls::TlsConnectorExt;
use tokio_io::codec::Framed;
use futures::sync::mpsc;
use base64::{encode};
use std::marker::PhantomData;
use tokio_tls::TlsStream;
use futures;


const START: &'static str = "<?xml version='1.0'?><stream:stream version='1.0' xmlns:stream='http://etherx.jabber.org/streams' to='example.com' xmlns='jabber:client'>";
const AUTH: &'static str = "<starttls xmlns='urn:ietf:params:xml:ns:xmpp-tls'/>";
const TLS_SUCCESS: &'static str = "
<stream:stream xmlns='jabber:client' xmlns:stream='http://etherx.jabber.org/streams' to='example.com' version='1.0'>";
const PLAIN: &'static str = "<auth xmlns='urn:ietf:params:xml:ns:xmpp-sasl' mechanism='PLAIN'>";


fn initialize_stream(t: Framed<TokioStream, LineCodec>) -> futures::AndThen {
    t.send(START.to_string())
        .and_then(|transport| transport.into_future().map_err(|(e, _)| e))
}
pub fn connect_client<F>(out_tx: mpsc::Sender<(ClientMessage, mpsc::Sender<ClientMessage>)>, f: F) 
    where F: Fn(ServerMessage) -> Option<ClientMessage> + 'static
{
    let (tx, rx) = mpsc::channel(1);
    let send_to_server = |msg| {
        match tx.clone().start_send(msg) {
            Ok(_) => println!("message sent"),
            Err(_) => println!("fail to send to sink")
        }
    };

    let mut core = Core::new().unwrap();

    let stream = TcpStream::connect(("xmpp-qa.iadvize.com", 5222)).unwrap();
    let socket = TokioStream::from_stream(stream, &core.handle()).unwrap();

    let transport = socket.framed(LineCodec);
        let starttls = |(_, t): (Option<String>, Framed<TokioStream, LineCodec>)| {
        t.send(AUTH.to_string())
            .and_then(|transport| transport.into_future().map_err(|(e, _)| e))
    };

    let negociate = |(_, transport): (Option<String>, Framed<TokioStream, LineCodec>)| {
        let builder = TlsConnector::builder().unwrap();
        let cx = builder.build().unwrap();

        println!("connected");
        // cx.connect_no_domain(transport.into_inner()).map_err(|e| {
        cx.connect_async("xmpp-qa.iadvize.com", transport.into_inner()).map_err(|e| {
            io::Error::new(io::ErrorKind::Other, e)
        })
    };

    let open_tls_stream = |socket: TlsStream<TokioStream>| {
        let transport = socket.framed(LineCodec);

        transport.send(TLS_SUCCESS.to_string())
            .and_then(|transport| transport.into_future().map_err(|(e, _)| e))
    };

    let socket = initialize_stream(transport)
        .and_then(starttls)
        .and_then(negociate)
        .and_then(open_tls_stream)
        .and_then(|(_, transport)| {
            let mut data: Vec<u8> = Vec::new();
            data.push(0);
            // data.extend(b"alice@example.com");
            data.extend(b"admin@iadvize.com");
            data.push(0);
            data.extend(b"iAdvize");

            // let plain = data.to_base64();

            let bytes = str::from_utf8(&data).unwrap().as_bytes();
            let plain = encode(bytes);
            let plain = format!("{}{}</auth>", PLAIN, plain);
            transport.send(plain)
        })
    .and_then(|transport| transport.into_future().map_err(|(e, _)| e))
        .and_then(|(_, transport)| {
            transport.send("<stream:stream xmlns='jabber:client' xmlns:stream='http://etherx.jabber.org/streams' to='example.com' version='1.0'>".to_string())
        })

    .and_then(|transport| transport.into_future().map_err(|(e, _)| e))
    .and_then(|(_, transport)| {
        transport.send("<iq type='set' id='bind_1'><bind xmlns='urn:ietf:params:xml:ns:xmpp-bind'/></iq>".to_string())
        })
    .and_then(|transport| transport.into_future().map_err(|(e, _)| e))
    .and_then(|(response, transport)| {
        println!("{:?}", response);
        let socket = transport.into_inner();
        let transport = socket.framed(ClientToServerCodec::new());

        let (to_server, from_server) = transport.split();
        let reader = from_server
            .for_each(move |msg| {
                match f(msg) {
                    Some(ret) => send_to_server(ret),
                    None => {}
                };

                Ok(())
            });

        let writer = rx
            .map_err(|()| unreachable!("rx can't fail"))
            .fold(to_server, |to_server, msg| {
                to_server.send(msg)
            })
        .map(|_| ());

        let _ = out_tx.clone().start_send((ClientMessage("connected".to_string()), tx.clone()));
        reader.select(writer).map(|_| ()).map_err(|(err, _)| err)

    });

    core.run(socket).unwrap();
}
/// Our line-based codec
pub struct LineCodec;
/// Implementation of the simple line-based protocol.
///
/// Frames consist of a UTF-8 encoded string, terminated by a '\n' character.
impl Decoder for LineCodec {
    type Item = String;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<String>, io::Error> {
        let len = buf.len();
        if len > 1 {
            let line = buf.split_to(len);

            return match str::from_utf8(&line.as_ref()) {
                Ok(s) => {
                    if s.starts_with("<?xml") {
                        let split = s.split("/><").collect::<Vec<&str>>();
                        if split.len() > 1 {
                            Ok(Some(split[0].to_string()))
                        } else {
                            Ok(None)
                        }
                    } else {
                        Ok(Some(s.to_string()))
                    }
                },
                Err(_) => Err(io::Error::new(io::ErrorKind::Other, "invalid string")),
            }
        }

        Ok(None)
    }
}

impl Encoder for LineCodec {
    type Item = String;
    type Error = io::Error;

    fn encode(&mut self, msg: String, buf: &mut BytesMut) -> io::Result<()> {
        buf.reserve(msg.len());

        buf.extend(msg.as_bytes());

        Ok(())
    }
}


#[derive(Debug, Clone)]
pub struct Handshake {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct ClientMessage(pub String);

#[derive(Debug)]
pub struct ServerMessage(pub String);


pub struct LengthPrefixedJson{
    _in: PhantomData<ServerMessage>,
    _out: PhantomData<ClientMessage>,
}

impl LengthPrefixedJson {
    pub fn new() -> LengthPrefixedJson{
        LengthPrefixedJson {
            _in: PhantomData,
            _out: PhantomData,
        }
    }
}

impl Decoder for LengthPrefixedJson
{
    type Item = ServerMessage;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, io::Error> {
        let len = buf.len();
        if len < 1 {
            return Ok(None);
        }

        let buf = buf.split_to(len);
        let s = str::from_utf8(buf.as_ref()).unwrap();

        println!("IN: {:?}", s);
        Ok(Some(ServerMessage(s.to_string())))
    }
}
impl Encoder for LengthPrefixedJson {
    type Item = ClientMessage;
    type Error = io::Error;


    fn encode(&mut self, msg: ClientMessage, buf: &mut BytesMut) -> io::Result<()> {
        println!("OUT: {:?}", msg.0);
        buf.extend(msg.0.as_bytes());

        Ok(())
    }
}

// pub type ServerToClientCodec = LengthPrefixedJson;
pub type ClientToServerCodec = LengthPrefixedJson;
