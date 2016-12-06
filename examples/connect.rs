extern crate xmpp;

use xmpp::stream::XmppStream;
use xmpp::stream::Event as XmppEvent;

fn main() {
    let mut connection = XmppStream::new("alice", "127.0.0.1", "test");

    match connection.connect() {
        Ok(_) => {
            loop {
                match connection.handle_event() {
                    XmppEvent::StreamClosed => break
                }
            }
        },
        Err(e) => panic!(e)
    }
}
