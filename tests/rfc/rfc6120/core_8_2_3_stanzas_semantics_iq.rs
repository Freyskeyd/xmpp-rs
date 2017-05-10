/// https://xmpp.org/rfcs/rfc6120.html#stanzas-semantics-iq

use xmpp_proto::events::GenericIq;
use xmpp_proto::events::IqType;
use elementtree::Element;
use std::io::ErrorKind;

mod id_is_required {
    use super::*;
    use std::error::Error;
    use xmpp_proto::events::FromXmlElement;

    #[test]
    fn valid() {
        let valid = r#"<iq from='capulet.lit' to='juliet@capulet.lit/balcony' id='s2c1' type='get'><ping xmlns='urn:xmpp:ping'/></iq>"#.as_bytes();

        match GenericIq::from_element(Element::from_reader(valid).unwrap()) {
            Ok(iq) => {
                assert_eq!(iq.get_id(), "s2c1");
            },
            Err(e) => {
                assert!(false, format!("{:?}", e));
            }
        }
    }

    #[test]
    fn invalid() {
        let invalid = r#"<iq from='capulet.lit' to='juliet@capulet.lit/balcony' type='get'><ping xmlns='urn:xmpp:ping'/></iq>"#.as_bytes();

        match GenericIq::from_element(Element::from_reader(invalid).unwrap()) {
            Err(e) => {
                assert!(e.kind() == ErrorKind::InvalidInput);
                assert!(e.description().contains("ID is required"));
            },
            Ok(e) => {
                assert!(false, format!("{:?}", e));
            }
        }
    }
}

mod type_is_required {
    use super::*;
    use std::error::Error;
    use xmpp_proto::events::FromXmlElement;

    #[test]
    fn get() {
        let valid = r#"<iq from='capulet.lit' to='juliet@capulet.lit/balcony' id='s2c1' type='get'><ping xmlns='urn:xmpp:ping'/></iq>"#.as_bytes();

        match GenericIq::from_element(Element::from_reader(valid).unwrap()) {
            Ok(iq) => {
                assert_eq!(iq.get_type(), IqType::Get);
            },
            Err(e) => {
                assert!(false, format!("{:?}", e));
            }
        }
    }

    #[test]
    fn set() {
        let valid = r#"<iq from='capulet.lit' to='juliet@capulet.lit/balcony' id='s2c1' type='set'><ping xmlns='urn:xmpp:ping'/></iq>"#.as_bytes();

        match GenericIq::from_element(Element::from_reader(valid).unwrap()) {
            Ok(iq) => {
                assert_eq!(iq.get_type(), IqType::Set);
            },
            Err(e) => {
                assert!(false, format!("{:?}", e));
            }
        }
    }

    #[test]
    fn result() {
        let valid = r#"<iq from='capulet.lit' to='juliet@capulet.lit/balcony' id='s2c1' type='result'><ping xmlns='urn:xmpp:ping'/></iq>"#.as_bytes();

        match GenericIq::from_element(Element::from_reader(valid).unwrap()) {
            Ok(iq) => {
                assert_eq!(iq.get_type(), IqType::Result);
            },
            Err(e) => {
                assert!(false, format!("{:?}", e));
            }
        }
    }

    #[test]
    fn error() {
        let valid = r#"<iq from='capulet.lit' to='juliet@capulet.lit/balcony' id='s2c1' type='error'><ping xmlns='urn:xmpp:ping'/><error type='cancel'></error></iq>"#.as_bytes();

        match GenericIq::from_element(Element::from_reader(valid).unwrap()) {
            Ok(iq) => {
                assert_eq!(iq.get_type(), IqType::Error);
            },
            Err(e) => {
                assert!(false, format!("{:?}", e));
            }
        }
    }

    #[test]
    fn missing() {
        let invalid = r#"<iq from='capulet.lit' to='juliet@capulet.lit/balcony' id='s2c1'><ping xmlns='urn:xmpp:ping'/></iq>"#.as_bytes();

        match GenericIq::from_element(Element::from_reader(invalid).unwrap()) {
            Err(e) => {
                assert!(e.kind() == ErrorKind::InvalidInput);
                assert!(e.description().contains("TYPE is required"));
            },
            Ok(e) => {
                assert!(false, format!("{:?}", e));
            }
        }
    }

    #[test]
    fn invalid() {
        let invalid = r#"<iq from='capulet.lit' to='juliet@capulet.lit/balcony' id='s2c1' type='bibi'><ping xmlns='urn:xmpp:ping'/></iq>"#.as_bytes();

        match GenericIq::from_element(Element::from_reader(invalid).unwrap()) {
            Err(e) => {
                assert!(e.kind() == ErrorKind::InvalidInput);
                assert!(e.description().contains("Unsupported IqType"));
            },
            Ok(e) => {
                assert!(false, format!("{:?}", e));
            }
        }
    }
}

mod child_get_set {
    use super::*;
    use std::error::Error;
    use xmpp_proto::events::FromXmlElement;

    #[test]
    fn get_valid() {
        let invalid = r#"<iq from='capulet.lit' to='juliet@capulet.lit/balcony' id='s2c1' type='get'><ping xmlns='urn:xmpp:ping'/></iq>"#.as_bytes();

        match GenericIq::from_element(Element::from_reader(invalid).unwrap()) {
            Err(e) => {
                assert!(false, format!("{:?}", e));
            },
            Ok(iq) => {
                assert_eq!(iq.get_type(), IqType::Get);
                assert!(true, format!("{:?}", iq));
            }
        }
    }

    #[test]
    fn get_invalid() {
        let invalid = r#"<iq from='capulet.lit' to='juliet@capulet.lit/balcony' id='s2c1' type='get'><ping xmlns='urn:xmpp:ping'/><none /></iq>"#.as_bytes();

        match GenericIq::from_element(Element::from_reader(invalid).unwrap()) {
            Err(e) => {
                assert!(e.kind() == ErrorKind::InvalidInput);
                assert!(e.description().contains("IqType Get/Set MUST contain one and only one child"));
            },
            Ok(iq) => {
                assert_eq!(iq.get_type(), IqType::Get);
                assert!(false, format!("{:?}", iq));
            }
        }
    }

    #[test]
    fn set_valid() {
        let invalid = r#"<iq from='capulet.lit' to='juliet@capulet.lit/balcony' id='s2c1' type='set'><ping xmlns='urn:xmpp:ping'/></iq>"#.as_bytes();

        match GenericIq::from_element(Element::from_reader(invalid).unwrap()) {
            Err(e) => {
                assert!(false, format!("{:?}", e));
            },
            Ok(iq) => {
                assert_eq!(iq.get_type(), IqType::Set);
                assert!(true, format!("{:?}", iq));
            }
        }
    }

    #[test]
    fn set_invalid() {
        let invalid = r#"<iq from='capulet.lit' to='juliet@capulet.lit/balcony' id='s2c1' type='set'><ping xmlns='urn:xmpp:ping'/><none /></iq>"#.as_bytes();

        match GenericIq::from_element(Element::from_reader(invalid).unwrap()) {
            Err(e) => {
                assert!(e.kind() == ErrorKind::InvalidInput);
                assert!(e.description().contains("IqType Get/Set MUST contain one and only one child"));
            },
            Ok(iq) => {
                assert_eq!(iq.get_type(), IqType::Set);
                assert!(false, format!("{:?}", iq));
            }
        }
    }
}

mod child_result {
    use super::*;
    use std::error::Error;
    use xmpp_proto::events::FromXmlElement;

    #[test]
    fn result_valid() {
        let invalid = r#"<iq from='capulet.lit' to='juliet@capulet.lit/balcony' id='s2c1' type='result'><ping xmlns='urn:xmpp:ping'/></iq>"#.as_bytes();

        match GenericIq::from_element(Element::from_reader(invalid).unwrap()) {
            Err(e) => {
                assert!(false, format!("{:?}", e));
            },
            Ok(iq) => {
                assert_eq!(iq.get_type(), IqType::Result);
                assert!(true, format!("{:?}", iq));
            }
        }
    }

    #[test]
    fn result_invalid() {
        let invalid = r#"<iq from='capulet.lit' to='juliet@capulet.lit/balcony' id='s2c1' type='result'><ping xmlns='urn:xmpp:ping'/><none /></iq>"#.as_bytes();

        match GenericIq::from_element(Element::from_reader(invalid).unwrap()) {
            Err(e) => {
                assert!(e.kind() == ErrorKind::InvalidInput);
                assert!(e.description().contains("An IQ stanza of type \"result\" MUST include zero or one child elements."));
            },
            Ok(iq) => {
                assert_eq!(iq.get_type(), IqType::Result);
                assert!(false, format!("{:?}", iq));
            }
        }
    }
}

mod child_error {
    use super::*;
    use std::error::Error;
    use xmpp_proto::events::FromXmlElement;

    #[test]
    fn error_valid() {
        let invalid = r#"<iq from='capulet.lit' to='juliet@capulet.lit/balcony' id='s2c1' type='error'><ping xmlns='urn:xmpp:ping'/><error type='cancel'></error></iq>"#.as_bytes();

        match GenericIq::from_element(Element::from_reader(invalid).unwrap()) {
            Err(e) => {
                assert!(false, format!("{:?}", e));
            },
            Ok(iq) => {
                assert_eq!(iq.get_type(), IqType::Error);
                assert!(true, format!("{:?}", iq));
            }
        }
    }

    #[test]
    fn error_invalid() {
        let invalid = r#"<iq from='capulet.lit' to='juliet@capulet.lit/balcony' id='s2c1' type='error'><ping xmlns='urn:xmpp:ping'/></iq>"#.as_bytes();

        match GenericIq::from_element(Element::from_reader(invalid).unwrap()) {
            Err(e) => {
                assert!(e.kind() == ErrorKind::InvalidInput);
                assert!(e.description().contains("An IQ stanza of type \"error\" SHOULD include the child element contained in the associated \"get\" or \"set\" and MUST include an <error/> child"));
            },
            Ok(iq) => {
                assert_eq!(iq.get_type(), IqType::Result);
                assert!(false, format!("{:?}", iq));
            }
        }
    }   
}
