extern crate xmpp;

use xmpp::stream::XmppStream;

#[test]
fn it_can_connect() {
    match XmppStream::new("simon", "127.0.0.1", "test").connect() {
        Ok(_)  => assert!(true),
        Err(_) => assert!(false)
    };
}
