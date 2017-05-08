use std::str::FromStr;
use xmpp_proto::ToJid;
use xmpp_proto::events::Ping;
use xmpp_proto::events::IqType;
use xml::reader::{EventReader,XmlEvent};
use elementtree::Element;

#[test]
fn parse_ping() {
    let p = "\
    <iq from='capulet.lit' to='juliet@capulet.lit/balcony' id='s2c1' type='get'>
    <ping xmlns='urn:xmpp:ping'/>
    </iq>";

    let ping = Ping::from_str(p).unwrap();
    assert_eq!(ping.get_type(), IqType::Get);
    assert_eq!(ping.get_to().unwrap(), &"juliet@capulet.lit/balcony".to_jid().unwrap());
    assert_eq!(ping.get_from().unwrap(), &"capulet.lit".to_jid().unwrap());
}

#[test]
#[should_panic]
fn parse_fail_ping() {
    let p = "\
    <iq from='capulet.lit' to='juliet@capulet.lit/balcony' id='s2c1' type='get'>
    </iq>";

    Ping::from_str(p).unwrap();
}

#[test]
fn parse_iq_result_ping() {
    let p = "<iq from='juliet@capulet.lit/balcony' to='capulet.lit' id='s2c1' type='result'/>";

    let ping = Ping::from_str(p).unwrap();
    assert_eq!(ping.get_type(), IqType::Result);
    assert_eq!(ping.get_to().unwrap(), &"capulet.lit".to_jid().unwrap());
    assert_eq!(ping.get_from().unwrap(), &"juliet@capulet.lit/balcony".to_jid().unwrap());
}

#[test]
#[should_panic]
fn parse_iq_result_fail_ping() {
    let p = "<iq from='juliet@capulet.lit/balcony' to='capulet.lit' id='s2c1' type='result'><something></something><body></body></iq>";

    match Ping::from_str(p) {
        Ok(_) => {},
        Err(e) => panic!("{:?}", e)
    }
}

#[test]
#[should_panic]
fn parse_iq_result_fail2_ping() {
    let p = "<iq from='juliet@capulet.lit/balcony' to='capulet.lit' id='s2c1' type='result'><body></body></iq>";

    match Ping::from_str(p) {
        Ok(_) => {},
        Err(e) => panic!("{:?}", e)
    }
}

#[test]
fn parse_iq_error_ping() {
    let p = "<iq from='juliet@capulet.lit/balcony' to='capulet.lit' id='s2c1' type='error'>
  <ping xmlns='urn:xmpp:ping'/>
  <error type='cancel'>
    <service-unavailable xmlns='urn:ietf:params:xml:ns:xmpp-stanzas'/>
  </error>
</iq>";

    match Ping::from_str(p) {
        Ok(ping) => {
            assert_eq!(ping.get_type(), IqType::Error);
            assert_eq!(ping.get_to().unwrap(), &"capulet.lit".to_jid().unwrap());
            assert_eq!(ping.get_from().unwrap(), &"juliet@capulet.lit/balcony".to_jid().unwrap());
        },
        Err(e) => panic!("{:?}", e)
    }
}

#[test]
#[should_panic]
fn parse_iq_error2_ping() {
    let p = "<iq from='juliet@capulet.lit/balcony' to='capulet.lit' id='s2c1' type='error'>
  <ping xmlns='urn:xmpp:ping'/>
</iq>";

    match Ping::from_str(p) {
        Err(e) => panic!("{:?}", e),
        _ => {}
    }
}

#[test]
fn parse_iq_error3_ping() {
  let p = "<iq from='montague.lit' to='capulet.lit' id='s2s1' type='error'>
  <ping xmlns='urn:xmpp:ping'/>
  <error type='cancel'>
    <service-unavailable xmlns='urn:ietf:params:xml:ns:xmpp-stanzas'/>
  </error>
</iq>";
  match Ping::from_str(p) {
    Ok(ping) => {
      assert_eq!(ping.get_type(), IqType::Error);
      assert_eq!(ping.get_to().unwrap(), &"capulet.lit".to_jid().unwrap());
      assert_eq!(ping.get_from().unwrap(), &"montague.lit".to_jid().unwrap());
    },
    Err(e) => panic!("{:?}", e)
  }
}

#[test]
fn parser_second() {
  let mut p = "<iq from='capulet.lit' to='juliet@capulet.lit/balcony' id='s2c1' type='get'>
    <ping xmlns='urn:xmpp:ping'/>
    </iq><stream:stream".as_bytes();

  loop {
    match Element::from_reader(&mut p) {
      Ok(_) => {
        break;
      },
      Err(_) => {
        break;
      }
    };
  }
}
