use actix_web::{App, HttpServer};
use std::env;
use xmpp::server::Server;

#[actix::main]
async fn main() {
    // std::thread::spawn(move || {
    //     let mut sys = actix_web::rt::System::new();
    //     // let srv = HttpServer::new(|| App::new().route("/ws/", web::get().to(index))).bind("127.0.0.1:8080").unwrap().run();
    //     let srv = HttpServer::new(|| App::new()).bind("127.0.0.1:8080").unwrap().run();
    //     let _ = sys.block_on(srv);
    // });

    // async fn main() {
    env::set_var("RUST_LOG", "actix=trace,xmpp_server=trace");
    env_logger::init();
    Server::build().cert("./server.crt").keys("./server.key").launch().await;
}
