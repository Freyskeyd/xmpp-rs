use actix::{Actor, Context};

/// Manage to route packet on a node
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

// /// Manage to route packet when server is the target
// pub struct LocalRouter {}
// /// Manage to route packet based on pattern
// pub struct RegisteredRouteManager {}
