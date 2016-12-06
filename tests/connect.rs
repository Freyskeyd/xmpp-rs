extern crate xmpp;

use std::io;

use xmpp::stream::XmppStream;
use xmpp::stream::XmppStreamStatus;

#[test]
fn it_can_connect() {
    let mut connection = XmppStream::new("simon", "127.0.0.1", "test");
    match connection.connect() {
        Ok(_) => assert!(connection.streamStatus.connected()),
        Err(_) => assert!(false)
    };

    let status: &XmppStreamStatus = &connection.streamStatus;
    assert!(connection.streamStatus.connected());
    assert!(status.connected());
}
