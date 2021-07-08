use crate::messages::system::UnregisterSession;
use crate::{
    messages::{
        system::RegistrationStatus,
        system::{RegisterSession, SessionCommand},
        SessionManagementPacketResult, SessionPacket,
    },
    parser::codec::XmppCodec,
    router::Router,
    sessions::{manager::SessionManager, Session},
};
use actix::{io::FramedWrite, prelude::*};
use jid::FullJid;
use log::trace;
use std::{io, pin::Pin, str::FromStr};
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
    pub(crate) fn new(id: usize, router: Addr<Router>, sink: FramedWrite<Packet, Pin<Box<dyn AsyncWrite + 'static>>, XmppCodec>, session: Addr<Session>) -> Self {
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

    fn started(&mut self, ctx: &mut Self::Context) {
        trace!("Starting TcpSession");
        let referer = ctx.address().recipient::<SessionCommand>();
        let jid: FullJid = FullJid::from_str("admin@localhost/test").unwrap();
        let fut = async move { SessionManager::from_registry().send(RegisterSession { jid, referer }).await.unwrap() };

        ctx.wait(fut.into_actor(self).map(|res, actor, ctx| {
            println!("{:?}", res);

            match res {
                Ok(_) => println!("OK"),
                Err(_) => {
                    ctx.stop();
                }
            }
        }));
    }

    fn stopping(&mut self, _ctx: &mut Self::Context) -> actix::Running {
        trace!("Stopping TcpSession");
        let jid: FullJid = FullJid::from_str("admin@localhost/test").unwrap();
        let _ = SessionManager::from_registry().try_send(UnregisterSession { jid });
        actix::Running::Stop
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        trace!("TcpSession Stopped");
    }
}

impl actix::io::WriteHandler<io::Error> for TcpSession {}

impl StreamHandler<Result<Packet, io::Error>> for TcpSession {
    fn handle(&mut self, packet: Result<Packet, io::Error>, ctx: &mut Context<Self>) {
        if let Ok(packet) = packet {
            let _ = self.session.try_send(SessionPacket {
                packet,
                referer: ctx.address().recipient(),
            });
        }
    }
}

impl Handler<SessionCommand> for TcpSession {
    type Result = Result<(), ()>;

    fn handle(&mut self, msg: SessionCommand, ctx: &mut Self::Context) -> Self::Result {
        println!("{:?}", msg);
        match msg.0 {
            crate::messages::system::SessionCommandAction::Kill => ctx.stop(),
        }
        Ok(())
    }
}

impl Handler<SessionManagementPacketResult> for TcpSession {
    type Result = ();

    fn handle(&mut self, msg: SessionManagementPacketResult, _ctx: &mut Self::Context) -> Self::Result {
        println!("{:?}", msg);

        let SessionManagementPacketResult { session_state, packets } = msg;
        trace!("SessionState is {:?}", session_state);

        packets.into_iter().for_each(|packet| {
            self.sink.write(packet);
        });
    }
}
