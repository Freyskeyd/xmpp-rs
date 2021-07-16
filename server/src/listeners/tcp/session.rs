use crate::messages::system::{PacketIn, PacketsOut};
use crate::sessions::state::StaticSessionState;
use crate::{parser::codec::XmppCodec, router::Router, sessions::Session};
use actix::{io::FramedWrite, prelude::*};
use log::trace;
use std::{io, pin::Pin};
use tokio::io::AsyncWrite;
use xmpp_proto::Packet;

pub(crate) struct TcpSession {
    _id: usize,
    _router: Addr<Router>,
    session: Addr<Session>,
    #[allow(dead_code)]
    sink: FramedWrite<Packet, Pin<Box<dyn AsyncWrite + 'static>>, XmppCodec>,
}

impl TcpSession {
    pub(crate) fn new(_state: StaticSessionState, id: usize, router: Addr<Router>, sink: FramedWrite<Packet, Pin<Box<dyn AsyncWrite + 'static>>, XmppCodec>, session: Addr<Session>) -> Self {
        Self {
            _id: id,
            _router: router,
            sink,
            session,
        }
    }
}

impl Actor for TcpSession {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        trace!("Starting TcpSession");
    }

    fn stopping(&mut self, _ctx: &mut Self::Context) -> actix::Running {
        trace!("Stopping TcpSession");
        actix::Running::Stop
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        trace!("TcpSession Stopped");
    }
}

impl actix::io::WriteHandler<io::Error> for TcpSession {}

impl StreamHandler<Result<Packet, io::Error>> for TcpSession {
    fn handle(&mut self, packet: Result<Packet, io::Error>, _ctx: &mut Context<Self>) {
        if let Ok(packet) = packet {
            let _ = self.session.try_send(PacketIn(packet));
        }
    }
}

impl Handler<PacketsOut> for TcpSession {
    type Result = Result<(), ()>;

    fn handle(&mut self, msg: PacketsOut, _ctx: &mut Self::Context) -> Self::Result {
        msg.0.into_iter().for_each(|packet| {
            self.sink.write(packet);
        });

        Ok(())
    }
}
