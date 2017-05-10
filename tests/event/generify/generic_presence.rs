use std::str::FromStr;
use xmpp_proto::events::ToXmlElement;
use xmpp_proto::events::PresenceType;
use xmpp_proto::Jid;
use xmpp_proto::events::Presence;
use elementtree::Element;
use elementtree::WriteOptions;
use xmpp_proto::events::FromXmlElement;

fn element_to_string(e: Element) -> String {
    let mut out: Vec<u8> = Vec::new();
    let options = WriteOptions::new().set_xml_prolog(None);

    e.to_writer_with_options(&mut out, options).unwrap();
    String::from_utf8(out).unwrap()
}

#[test]
fn create_a_presence() {
    let mut g = Presence::new();

    let _ = g.set_to(Some("test@example.com"));
    // Presence can have a TO
    match g.get_to() {
        Some(to) => {
            assert_eq!(&Jid::from_str("test@example.com").unwrap(), to);
            assert_eq!("test@example.com", to.to_string())
        }
        None => {}
    }
    // Presence can have a TYPE
    let _ = g.set_type(Some(PresenceType::Available));
    match g.get_type() {
        None => assert!(false),
        Some(t) => {
            match *t {
                PresenceType::Available => assert!(true),
            }
        }
    }

    let _ = g.set_from(Some("test@example.com"));
    // Presence can have a FROM
    match g.get_from() {
        Some(from) => {
            assert_eq!(&Jid::from_str("test@example.com").unwrap(), from);
            assert_eq!("test@example.com", from.to_string())
        }
        None => {}
    }
}

#[test]
fn check_send_first_presence() {
    let first = Element::from_reader(r#"<presence />"#.as_bytes()).unwrap();

    if let Ok(presence) = Presence::from_element(first) {
        assert_eq!(presence.get_type(), None);
        assert_eq!(element_to_string(presence.to_element().unwrap()),
                   "<presence />");
    } else {
        panic!("");
    }
}

#[test]
fn build_first_presence() {
    let p = Presence::new();

    assert_eq!(element_to_string(p.to_element().unwrap()), "<presence />");
}
