use xmpp_proto::ToJid;
use xmpp_proto::events::Ping;
use xmpp_proto::events::IqType;
use xmpp_proto::events::FromXmlElement;
use xmpp_proto::Element;

#[test]
fn parse_ping() {
    let p = Element::from_reader(r#"<iq from='capulet.lit' to='juliet@capulet.lit/balcony' id='s2c1' type='get'><ping xmlns='urn:xmpp:ping'/></iq>"#.as_bytes()).unwrap();

    if let Ok(ping) = Ping::from_element(p) {
        assert_eq!(ping.get_type(), IqType::Get);
        assert_eq!(ping.get_to().unwrap(),
                   &"juliet@capulet.lit/balcony".to_jid().unwrap());
        assert_eq!(ping.get_from().unwrap(), &"capulet.lit".to_jid().unwrap());
    } else {
        assert!(false);
    }
}

#[test]
fn parse_fail_ping() {
    let p = Element::from_reader(r#"<iq from='capulet.lit' to='juliet@capulet.lit/balcony' id='s2c1' type='get'></iq>"#.as_bytes()).unwrap();

    match Ping::from_element(p) {
        Err(_) => assert!(true),
        _ => assert!(false)
    }
}

#[test]
fn parse_iq_result_ping() {
    let p = Element::from_reader(r#"<iq from='juliet@capulet.lit/balcony' to='capulet.lit' id='s2c1' type='result'/>"#.as_bytes()).unwrap();

    if let Ok(ping) = Ping::from_element(p) {
        assert_eq!(ping.get_type(), IqType::Result);
        assert_eq!(ping.get_to().unwrap(), &"capulet.lit".to_jid().unwrap());
        assert_eq!(ping.get_from().unwrap(),
                   &"juliet@capulet.lit/balcony".to_jid().unwrap());
    } else {
        assert!(false);
    }
}

#[test]
fn parse_iq_result_fail_ping() {
    let p = Element::from_reader(r#"<iq from='juliet@capulet.lit/balcony' to='capulet.lit' id='s2c1' type='result'><something></something><body></body></iq>"#.as_bytes()).unwrap();

    match Ping::from_element(p) {
        Ok(_) => assert!(false),
        Err(_) => assert!(true),
    }
}

#[test]
#[ignore]
fn parse_iq_result_fail2_ping() {
    let p = Element::from_reader(r#"<iq from='juliet@capulet.lit/balcony' to='capulet.lit' id='s2c1' type='result'><body></body></iq>"#.as_bytes()).unwrap();

    let _ = Ping::from_element(p);
}

#[test]
fn parse_iq_error_ping() {
    let p = Element::from_reader(r#"<iq from='juliet@capulet.lit/balcony' to='capulet.lit' id='s2c1' type='error'><ping xmlns='urn:xmpp:ping'/><error type='cancel'><service-unavailable xmlns='urn:ietf:params:xml:ns:xmpp-stanzas'/></error></iq>"#.as_bytes()).unwrap();

    if let Ok(ping) = Ping::from_element(p) {
        assert_eq!(ping.get_type(), IqType::Error);
        assert_eq!(ping.get_to().unwrap(), &"capulet.lit".to_jid().unwrap());
        assert_eq!(ping.get_from().unwrap(),
                   &"juliet@capulet.lit/balcony".to_jid().unwrap());
    } else {
        assert!(false);
    }
}

#[test]
fn parse_iq_error2_ping() {
    let p = Element::from_reader(r#"<iq from='juliet@capulet.lit/balcony' to='capulet.lit' id='s2c1' type='error'><ping xmlns='urn:xmpp:ping'/></iq>"#.as_bytes()).unwrap();

    match Ping::from_element(p) {
        Err(_) => assert!(true),
        _ => assert!(false)
    }
}

#[test]
fn parse_iq_error3_ping() {
    let p = Element::from_reader(r#"<iq from='montague.lit' to='capulet.lit' id='s2s1' type='error'><ping xmlns='urn:xmpp:ping'/><error type='cancel'><service-unavailable xmlns='urn:ietf:params:xml:ns:xmpp-stanzas'/></error></iq>"#.as_bytes()).unwrap();

    if let Ok(ping) = Ping::from_element(p) {
        assert_eq!(ping.get_type(), IqType::Error);
        assert_eq!(ping.get_to().unwrap(), &"capulet.lit".to_jid().unwrap());
        assert_eq!(ping.get_from().unwrap(), &"montague.lit".to_jid().unwrap());
    } else {
        assert!(false);
    }
}

#[test]
fn parse_iq_error4_ping() {
    let p = Element::from_reader(r#"<iq from='montague.lit' to='capulet.lit' id='s2s1' type='get'><ping xmlns='urn:xmpp:ping'/><error type='cancel'><service-unavailable xmlns='urn:ietf:params:xml:ns:xmpp-stanzas'/></error></iq>"#.as_bytes()).unwrap();

    if let Ok(_) = Ping::from_element(p) {
        assert!(false);
    } else {
        assert!(true);
    }
}

#[test]
fn parse_iq_error5_ping() {
    let p = Element::from_reader(r#"<iq from='montague.lit' to='capulet.lit' id='s2s1' type='error'><ping xmlns='urn:xmpp:ping'/><error type='bibi'><service-unavailable xmlns='urn:ietf:params:xml:ns:xmpp-stanzas'/></error></iq>"#.as_bytes()).unwrap();

    match Ping::from_element(p) {
        Ok(_) => assert!(false),
        Err(_) => assert!(true)
    }
}

#[test]
fn parse_iq_error6_ping() {
    let p = Element::from_reader(r#"<iq from='montague.lit' to='capulet.lit' id='s2s1' type='error'><ping xmlns='urn:xmpp:ping'/><error><service-unavailable xmlns='urn:ietf:params:xml:ns:xmpp-stanzas'/></error></iq>"#.as_bytes()).unwrap();

    if let Ok(_) = Ping::from_element(p) {
        assert!(false);
    } else {
        assert!(true);
    }
}
