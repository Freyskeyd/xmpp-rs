use xmpp_proto::XmppParser;
use xmpp_proto::events::Event::NonStanza;
use xmpp_proto::events::StreamFeatures;

#[test]
fn features_starttls() {
  let mut x = XmppParser::new();

  x.feed("<?xml version='1.0'?><stream:stream xml:lang='en' xmlns:stream='http://etherx.jabber.org/streams'>".as_bytes());
  match x.next_event() {
    None => assert!(false),
    Some(_) => assert!(true)
  }

  let test_str = "<stream:features><starttls xmlns=\"urn:ietf:params:xml:ns:xmpp-tls\"><required /></starttls></stream:features>";
  x.feed(&test_str.as_bytes());

  let e = match x.next_event() {
    None => panic!(false),
    Some(e) => e
  };

  match e {
    NonStanza(e, s) => {
      assert_eq!(s, test_str);
    },
    _ => panic!("")
  }
}

#[test]
fn features_bind() {
  let mut x = XmppParser::new();

  x.feed("<?xml version='1.0'?><stream:stream xml:lang='en' xmlns:stream='http://etherx.jabber.org/streams'>".as_bytes());
  match x.next_event() {
    None => assert!(false),
    Some(_) => assert!(true)
  }

  let test_str = "<stream:features><bind xmlns=\"urn:ietf:params:xml:ns:xmpp-bind\" /><session xmlns=\"urn:ietf:params:xml:ns:xmpp-session\" xmlns:stream=\"http://etherx.jabber.org/streams\"><optional /></session></stream:features>";
  x.feed(&test_str.as_bytes());

  let e = match x.next_event() {
    None => panic!(false),
    Some(e) => e
  };

  match e {
    NonStanza(e, s) => {
      assert_eq!(s, test_str);
    },
    _ => panic!("")
  }
}

#[test]
fn features_mechanisms() {
  let mut x = XmppParser::new();

  x.feed("<?xml version='1.0'?><stream:stream xml:lang='en' xmlns:stream='http://etherx.jabber.org/streams'>".as_bytes());
  match x.next_event() {
    None => assert!(false),
    Some(_) => assert!(true)
  }

  let test_str = "<stream:features><mechanisms xmlns=\"urn:ietf:params:xml:ns:xmpp-sasl\"><mechanism>PLAIN</mechanism><mechanism>DIGEST-MD5</mechanism><mechanism>X-OAUTH2</mechanism><mechanism>SCRAM-SHA-1</mechanism></mechanisms></stream:features>";
  x.feed(&test_str.as_bytes());

  let e = match x.next_event() {
    None => panic!(false),
    Some(e) => e
  };

  match e {
    NonStanza(e, s) => {
      assert_eq!(s, test_str);
    },
    _ => panic!("")
  }
}
