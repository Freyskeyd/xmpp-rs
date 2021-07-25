use crate::messages::system::{PacketIn, PacketsOut, SessionCommand, SessionCommandAction};
use crate::sessions::state::{SessionState, StaticSessionState};
use crate::{parser::codec::XmppCodec, sessions::Session};
use actix::{io::FramedWrite, prelude::*};
use log::trace;
use std::time::Duration;
use std::{io, pin::Pin};
use tokio::io::AsyncWrite;
use xmpp_proto::Packet;

pub(crate) struct TcpSession {
    _id: usize,
    session: Addr<Session>,
    state: SessionState,
    #[allow(dead_code)]
    sink: FramedWrite<Packet, Pin<Box<dyn AsyncWrite + 'static>>, XmppCodec>,
    timeout_handler: Option<SpawnHandle>,
}

impl TcpSession {
    pub(crate) fn new(_state: StaticSessionState, id: usize, sink: FramedWrite<Packet, Pin<Box<dyn AsyncWrite + 'static>>, XmppCodec>, session: Addr<Session>) -> Self {
        Self {
            _id: id,
            state: SessionState::Opening,
            sink,
            session,
            timeout_handler: None,
        }
    }

    fn reset_timeout(&mut self, ctx: &mut <Self as Actor>::Context) {
        if let Some(handler) = self.timeout_handler {
            if ctx.cancel_future(handler) {
                trace!("Timeout handler resetted for session");
            } else {
                trace!("Unable to reset timeout handler for session");
                ctx.set_mailbox_capacity(0);
                self.state = SessionState::Closing;
                ctx.stop()
            }
        }

        self.timeout_handler = Some(ctx.run_later(Duration::from_secs(120), move |actor, ctx| {
            trace!("Duration ended");
            ctx.set_mailbox_capacity(0);
            actor.state = SessionState::Closing;
            let fut = actor.session.send(SessionCommand(SessionCommandAction::Kill)).into_actor(actor).map(|_, _, _| ());
            ctx.wait(fut);
            // ctx.stop();
        }));
    }
}

impl Actor for TcpSession {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        trace!("Starting TcpSession");
    }

    fn stopping(&mut self, _ctx: &mut Self::Context) -> actix::Running {
        trace!("Stopping TcpSession");
        if self.state != SessionState::Closing {
            self.session.do_send(SessionCommand(SessionCommandAction::Kill));
        }
        actix::Running::Stop
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        trace!("TcpSession Stopped");
    }
}

impl actix::io::WriteHandler<io::Error> for TcpSession {}

impl StreamHandler<Result<Packet, io::Error>> for TcpSession {
    fn handle(&mut self, packet: Result<Packet, io::Error>, ctx: &mut Context<Self>) {
        self.reset_timeout(ctx);

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
