use actix::{io::FramedWrite, Actor, Addr, Context, StreamHandler};
use std::io;
use tokio::io::WriteHalf;
use tokio::net::TcpStream;
use tokio_rustls::server::TlsStream;

use crate::{router::Router, XmppCodec};
use xmpp_proto::{Features, NonStanza, OpenStream, Packet, ProceedTls, Stanza, StreamFeatures};

pub(crate) struct TcpSession {
    _id: usize,
    _router: Addr<Router>,
    sink: FramedWrite<Packet, WriteHalf<TlsStream<TcpStream>>, XmppCodec>,
    state: SessionState,
}

enum SessionState {
    Initialized,
    Authenticated,
}

impl TcpSession {
    pub(crate) fn new(id: usize, router: Addr<Router>, sink: FramedWrite<Packet, WriteHalf<TlsStream<TcpStream>>, XmppCodec>) -> Self {
        Self {
            _id: id,
            _router: router,
            sink,
            state: SessionState::Initialized,
        }
    }
}

impl Actor for TcpSession {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("Starting TcpSession");
    }

    fn stopping(&mut self, _ctx: &mut Self::Context) -> actix::Running {
        println!("Stopping TcpSession");
        actix::Running::Stop
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        println!("TcpSession Stopped");
    }
}
impl actix::io::WriteHandler<io::Error> for TcpSession {}

impl StreamHandler<Result<Packet, io::Error>> for TcpSession {
    fn handle(&mut self, msg: Result<Packet, io::Error>, _ctx: &mut Context<Self>) {
        match msg {
            Ok(Packet::NonStanza(NonStanza::OpenStream(OpenStream { to, xmlns, lang, version, from, id }))) => match self.state {
                SessionState::Initialized => {
                    self.sink.write(Packet::NonStanza(NonStanza::OpenStream(OpenStream {
                        id,
                        to: from,
                        from: to,
                        xmlns,
                        lang,
                        version,
                    })));
                    self.sink.write(Packet::NonStanza(NonStanza::StreamFeatures(StreamFeatures {
                        features: Features::Mechanisms(vec!["PLAIN".to_string()]),
                    })));
                }
                SessionState::Authenticated => {
                    self.sink.write(Packet::NonStanza(NonStanza::OpenStream(OpenStream {
                        id,
                        to: from,
                        from: to,
                        xmlns,
                        lang,
                        version,
                    })));

                    self.sink.write(Packet::NonStanza(NonStanza::StreamFeatures(StreamFeatures { features: Features::Bind })));
                }
            },
            Ok(Packet::NonStanza(NonStanza::ProceedTls(ProceedTls { .. }))) => {
                self.state = SessionState::Authenticated;
                self.sink.write(Packet::NonStanza(NonStanza::SASLSuccess))
            }
            Ok(Packet::NonStanza(NonStanza::SASLSuccess)) => self.sink.write(Packet::NonStanza(NonStanza::SASLSuccess)),

            Ok(Packet::Stanza(Stanza::IQ(e))) => {
                println!("Attributes : {:?}", e);
            }

            _ => (),
        };
    }
}
