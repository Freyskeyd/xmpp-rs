use actix::Actor;

pub(crate) mod manager;

pub trait IqHandler: Actor {
    fn register();
}
