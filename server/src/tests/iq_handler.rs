use actix::{Actor, Context};
use demonstrate::demonstrate;

#[derive(Default)]
struct MyIQHandler {}

impl Actor for MyIQHandler {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {}
}

demonstrate! {
    describe "when starting an IQHandler" {
        use super::*;

        before {}

        #[actix::test]
        async it "should register to the Manager" -> Result<(), ()> {
            MyIQHandler::default().start();
            Ok(())
        }

    }
}
