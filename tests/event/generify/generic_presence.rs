use std::str::FromStr;
use xmpp_proto::events::PresenceType;
use xmpp_proto::{Jid};
use xmpp_proto::events::Presence;

#[test]
fn create_a_presence() {
    let mut g = Presence::new();

    let _ = g.set_to(Some("test@example.com"));
    // Presence can have a TO
    match g.get_to() {
        Some(to) => {
            assert_eq!(&Jid::from_str("test@example.com").unwrap(), to);
            assert_eq!("test@example.com", to.to_string())
        },
        None => {}
    }
    // Presence can have a TYPE
    let _ = g.set_type(Some(PresenceType::Available));
    match g.get_type() {
        None => assert!(false),
        Some(t) => match *t {
            PresenceType::Available => assert!(true)
        }
    }

    let _ = g.set_from(Some("test@example.com"));
    // Presence can have a FROM
    match g.get_from() {
        Some(from) => {
            assert_eq!(&Jid::from_str("test@example.com").unwrap(), from);
            assert_eq!("test@example.com", from.to_string())
        },
        None => {}
    }
}

#[test]
fn check_send_first_presence() {
    let first = "<presence />";

    match Presence::from_str(first) {
        Ok(presence) => {
            assert_eq!(presence.get_type(), None);
            assert_eq!(presence.to_string(), "<presence />");
        },
        Err(_) => assert!(false)
    };
}

#[test]
fn build_first_presence() {
    let p = Presence::new();

    assert_eq!(p.to_string(), "<presence />");
}
