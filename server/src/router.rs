use actix::{Actor, Context};

#[derive(Debug)]
pub struct Router {}

impl Router {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl Actor for Router {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("Router started");
    }
}
