use std::env;
use xmpp::server::Server;

#[actix::main]
async fn main() {
    env::set_var("RUST_LOG", "actix=trace,xmpp_server=trace");
    env_logger::init();
    Server::build().cert("./server.crt").keys("./server.key").launch().await;
}
