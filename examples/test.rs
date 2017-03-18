extern crate xmpp;

use xmpp::connect_client;
fn main() {
    connect_client()
    // let (tx, rx) = mpsc::channel(1);

    // run_client(rx, tx, |ServerMessage(msg)| -> Option<ClientMessage> {
    //     if msg.len() > 1 {
    //         if msg.starts_with("<proceed") {
    //             return Some(ClientMessage("<auth".to_string()));
    //         }
    //     }
    //     None
    // });
}


