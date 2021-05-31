use actix::{Actor, ActorContext, StreamHandler};
use actix_web_actors::ws;

/// websocket connection is long running connection, it easier
/// to handle with an actor
struct MyWebSocket {
    // hb: Instant,
}

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;

    // fn started(&mut self, ctx: &mut Self::Context) {
    //     self.hb(ctx);
    // }
}

/// Handler for `ws::Message`
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        // process websocket messages
        match msg {
            // Ok(ws::Message::Ping(msg)) => {
            //     self.hb = Instant::now();
            //     ctx.pong(&msg);
            // }
            // Ok(ws::Message::Pong(_)) => {
            //     self.hb = Instant::now();
            // }
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}
