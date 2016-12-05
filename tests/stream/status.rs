extern crate xmpp;

use xmpp::stream::XmppStreamStatus;

#[test]
fn it_should_be_instiable() {
    let status = XmppStreamStatus::new();

    assert!(status.connected());
    connection.disconnect();
}
