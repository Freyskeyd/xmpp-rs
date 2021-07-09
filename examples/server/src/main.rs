use std::env;
use xmpp::server::Server;

#[actix::main]
async fn main() {
    env::set_var("RUST_LOG", "actix=trace,xmpp_server=trace,xmpp_server::parser::codec=info");
    env_logger::init();

    let _ = Server::build().launch().await;
}
