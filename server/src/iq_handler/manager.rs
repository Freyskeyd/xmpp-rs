use actix::{Actor, Context, Supervised, SystemService};
use log::trace;

#[derive(Default, Debug)]
pub(crate) struct IqHandlerManager {}

impl Supervised for IqHandlerManager {}
impl SystemService for IqHandlerManager {}

impl Actor for IqHandlerManager {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        trace!("IqHandlerManager started");
    }
}
