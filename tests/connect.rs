extern crate xmpp;

use xmpp::stream::XmppStream;

#[test]
fn it_can_connect() {
    let connection = XmppStream::new("simon", "127.0.0.1", "test");
    let status = connection.connect();

    assert!(status.connected());
    connection.disconnect();
}
