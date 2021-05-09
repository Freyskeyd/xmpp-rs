// use circular::Buffer;
// use std::io::Write;
// use xml::ParserConfig;
// use xmpp_proto::{NonStanza, Packet};

// #[test]
// fn features_starttls() {
//     let mut cfg = ParserConfig::new().whitespace_to_characters(true);
//     cfg.ignore_end_of_stream = true;
//     let mut reader = cfg.create_reader(Buffer::with_capacity(4096));

//     reader
//         .source_mut()
//         .write("<stream:stream xml:lang='en' xmlns:stream='http://etherx.jabber.org/streams'>".as_bytes())
//         .unwrap();

//     let _ = reader.next().unwrap();
//     let x = reader.next().unwrap();
//     let packet = match x {
//         xml::reader::XmlEvent::StartElement { name, attributes, namespace } => Packet::parse(&mut reader, name, namespace, attributes),
//         e => panic!("{:?}", e),
//     };

//     assert!(
//         matches!(packet, Some(Packet::NonStanza(ref stanza)) if matches!(**stanza, NonStanza::OpenStream(_))),
//         "Packet wasn't an OpenStream, it was: {:?}",
//         packet
//     );
// }

// #[test]
// fn features_bind() {
//     let mut cfg = ParserConfig::new().whitespace_to_characters(true);
//     cfg.ignore_end_of_stream = true;
//     let mut reader = cfg.create_reader(Buffer::with_capacity(4096));

//     reader
//         .source_mut()
//         .write("<stream:features><bind xmlns=\"urn:ietf:params:xml:ns:xmpp-bind\" /><session xmlns=\"urn:ietf:params:xml:ns:xmpp-session\" xmlns:stream=\"http://etherx.jabber.org/streams\"><optional /></session></stream:features>".as_bytes())
//         .unwrap();

//     let _ = reader.next().unwrap();
//     let x = reader.next().unwrap();
//     let packet = match x {
//         xml::reader::XmlEvent::StartElement { name, attributes, namespace } => Packet::parse(&mut reader, name, namespace, attributes),
//         e => panic!("{:?}", e),
//     };

//     assert!(
//         matches!(packet, Some(Packet::NonStanza(ref stanza)) if matches!(**stanza, NonStanza::OpenStream(_))),
//         "Packet wasn't an StreamFeature Bind, it was: {:?}",
//         packet
//     );
// }

// #[test]
// fn features_mechanisms() {
//     let mut x = XmppParser::new();

//     x.feed("<?xml version='1.0'?><stream:stream xml:lang='en' xmlns:stream='http://etherx.jabber.org/streams'>".as_bytes());
//     match x.next_event() {
//         None => assert!(false),
//         Some(_) => assert!(true),
//     }

//     let test_str = "<stream:features><mechanisms xmlns=\"urn:ietf:params:xml:ns:xmpp-sasl\"><mechanism>PLAIN</mechanism><mechanism>DIGEST-MD5</mechanism><mechanism>X-OAUTH2</mechanism><mechanism>SCRAM-SHA-1</mechanism></mechanisms></stream:features>";
//     x.feed(&test_str.as_bytes());

//     let e = match x.next_event() {
//         None => panic!(false),
//         Some(e) => e,
//     };

//     match e {
//         NonStanza(e) => match *e {
//             StreamFeaturesEvent(event) => {
//                 let s = element_to_string(event.to_element().unwrap());
//                 assert_eq!(s, test_str);
//             }
//             _ => assert!(false),
//         },
//         _ => assert!(false),
//     }
// }
