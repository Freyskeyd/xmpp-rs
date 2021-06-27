use crate::{parser::codec::XmppCodec, sessions::manager::RegistrationStatus};
use crate::{
    router::Router,
    sessions::manager::{RegisterSession, SessionManager},
};
use actix::{io::FramedWrite, prelude::*};
use log::trace;
use std::{io, pin::Pin};
use tokio::io::AsyncWrite;
use xmpp_proto::Packet;

pub(crate) struct TcpSession {
    _id: usize,
    _router: Addr<Router>,
    #[allow(dead_code)]
    sink: FramedWrite<Packet, Pin<Box<dyn AsyncWrite + 'static>>, XmppCodec>,
}

impl TcpSession {
    pub(crate) fn new(id: usize, router: Addr<Router>, sink: FramedWrite<Packet, Pin<Box<dyn AsyncWrite + 'static>>, XmppCodec>) -> Self {
        Self { _id: id, _router: router, sink }
    }
}

impl Actor for TcpSession {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        trace!("Starting TcpSession");
        let referer = ctx.address().recipient::<RegistrationStatus>();
        let fut = async move {
            let _ = SessionManager::from_registry().send(RegisterSession { referer: referer }).await;
        };

        ctx.wait(fut.into_actor(self));
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
    fn handle(&mut self, msg: Result<Packet, io::Error>, _ctx: &mut Context<Self>) {
        println!("{:?}", msg);
    }
}
impl Handler<RegistrationStatus> for TcpSession {
    type Result = Result<(), ()>;

    fn handle(&mut self, msg: RegistrationStatus, _ctx: &mut Self::Context) -> Self::Result {
        println!("{:?}", msg);
        Ok(())
    }
}
