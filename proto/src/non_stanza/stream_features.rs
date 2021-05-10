use xmpp_xml::Element;

use crate::{ns, FromXmlElement, NonStanza, Packet, ToXmlElement};

#[derive(derive_builder::Builder, Debug, Clone)]
#[builder(setter(into))]
pub struct StreamFeatures {
    pub features: Features,
}

impl FromXmlElement for StreamFeatures {
    type Error = std::io::Error;

    fn from_element(e: Element) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        Ok(Self {
            features: e
                .children()
                .map(|child| match Features::from(child.tag().name()) {
                    Features::Mechanisms(_) => Features::Mechanisms(child.children().map(|m| m.text().to_string()).collect()),
                    feature => feature,
                })
                .collect::<Vec<Features>>()
                .first()
                .cloned()
                .unwrap_or(Features::Unknown),
        })
    }
}
impl From<StreamFeatures> for Packet {
    fn from(s: StreamFeatures) -> Self {
        NonStanza::StreamFeatures(s).into()
    }
}

impl ToXmlElement for StreamFeatures {
    type Error = std::io::Error;
    fn to_element(&self) -> Result<Element, std::io::Error> {
        let mut root = Element::new("stream:features");

        match self.features {
            Features::StartTls => {
                let starttls = root.append_new_child((ns::TLS, "starttls"));
                starttls.append_new_child((ns::TLS, "required"));
            }
            Features::Bind => {
                root.append_new_child((ns::BIND, "bind"));
            }
            Features::Mechanisms(ref mechanisms) => {
                let node = root.append_new_child((ns::SASL, "mechanisms"));
                mechanisms.iter().for_each(|mech| {
                    node.append_new_child((ns::SASL, "mechanism")).set_text(mech);
                });
            }
            Features::Unknown => {}
        }

        Ok(root)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Features {
    StartTls,
    Bind,
    Mechanisms(Vec<String>),
    Unknown,
}

impl From<&str> for Features {
    fn from(e: &str) -> Self {
        match e {
            "starttls" => Features::StartTls,
            "bind" => Features::Bind,
            "mechanisms" => Features::Mechanisms(vec![]),
            _ => Features::Unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use circular::Buffer;
    use std::io::Write;

    use xmpp_xml::{
        xml::{reader::XmlEvent, ParserConfig},
        WriteOptions,
    };

    const INITIAL_STREAM: &'static str = "<stream:stream version='1.0' xmlns:stream='http://etherx.jabber.org/streams' to='example.com' xmlns='jabber:client'>";
    const EXPECTED_STARTTLS: &'static str = r#"<stream:features><starttls xmlns="urn:ietf:params:xml:ns:xmpp-tls"><required /></starttls></stream:features>"#;
    const EXPECTED_BIND: &'static str = r#"<stream:features><bind xmlns="urn:ietf:params:xml:ns:xmpp-bind" /></stream:features>"#;
    const EXPECTED_MECHANISMS: &'static str = r#"<stream:features><mechanisms xmlns="urn:ietf:params:xml:ns:xmpp-sasl"><mechanism>EXTERNAL</mechanism><mechanism>SCRAM-SHA-1-PLUS</mechanism><mechanism>SCRAM-SHA-1</mechanism><mechanism>PLAIN</mechanism></mechanisms></stream:features>"#;

    #[test]
    fn parse_starttls() {
        let stream_feature = StreamFeaturesBuilder::default().features(Features::StartTls).build();

        let mut output: Vec<u8> = Vec::new();
        let _ = stream_feature
            .unwrap()
            .to_element()
            .unwrap()
            .to_writer_with_options(&mut output, WriteOptions::new().set_xml_prolog(None));

        let generated = String::from_utf8(output).unwrap();

        assert!(EXPECTED_STARTTLS == generated);

        let mut cfg = ParserConfig::new().whitespace_to_characters(true);
        cfg.ignore_end_of_stream = true;
        let mut reader = cfg.create_reader(Buffer::with_capacity(4096));

        reader.source_mut().write(INITIAL_STREAM.as_bytes()).unwrap();
        reader.source_mut().write(EXPECTED_STARTTLS.as_bytes()).unwrap();

        let _ = reader.next().unwrap();
        let _ = reader.next().unwrap();
        let x = reader.next().unwrap();

        assert!(matches!(x, XmlEvent::StartElement { .. }));

        if let XmlEvent::StartElement { name, attributes, namespace } = x {
            let packet = Packet::parse(&mut reader, name, namespace, attributes);
            assert!(
                matches!(packet, Some(Packet::NonStanza(ref stanza)) if matches!(**stanza, NonStanza::StreamFeatures(StreamFeatures { features: Features::StartTls }))),
                "Packet wasn't an StreamFeatures::StartTls, it was: {:?}",
                packet
            );
        }
    }

    #[test]
    fn parse_bind() {
        let stream_feature = StreamFeaturesBuilder::default().features(Features::Bind).build();

        let mut output: Vec<u8> = Vec::new();
        let _ = stream_feature
            .unwrap()
            .to_element()
            .unwrap()
            .to_writer_with_options(&mut output, WriteOptions::new().set_xml_prolog(None));

        let generated = String::from_utf8(output).unwrap();

        assert!(EXPECTED_BIND == generated);

        let mut cfg = ParserConfig::new().whitespace_to_characters(true);
        cfg.ignore_end_of_stream = true;
        let mut reader = cfg.create_reader(Buffer::with_capacity(4096));

        reader.source_mut().write(INITIAL_STREAM.as_bytes()).unwrap();
        reader.source_mut().write(EXPECTED_BIND.as_bytes()).unwrap();

        let _ = reader.next().unwrap();
        let _ = reader.next().unwrap();
        let x = reader.next().unwrap();

        assert!(matches!(x, XmlEvent::StartElement { .. }));

        if let XmlEvent::StartElement { name, attributes, namespace } = x {
            let packet = Packet::parse(&mut reader, name, namespace, attributes);
            assert!(
                matches!(packet, Some(Packet::NonStanza(ref stanza)) if matches!(**stanza, NonStanza::StreamFeatures(StreamFeatures { features: Features::Bind }))),
                "Packet wasn't an StreamFeatures::Bind, it was: {:?}",
                packet
            );
        }
    }

    #[test]
    fn parse_mechanisms() {
        let mechs: Vec<String> = "EXTERNAL SCRAM-SHA-1-PLUS SCRAM-SHA-1 PLAIN".split_whitespace().map(String::from).collect();
        let stream_feature = StreamFeaturesBuilder::default().features(Features::Mechanisms(mechs.clone())).build();

        let mut output: Vec<u8> = Vec::new();
        let _ = stream_feature
            .unwrap()
            .to_element()
            .unwrap()
            .to_writer_with_options(&mut output, WriteOptions::new().set_xml_prolog(None));

        let generated = String::from_utf8(output).unwrap();

        assert!(EXPECTED_MECHANISMS == generated);

        let mut cfg = ParserConfig::new().whitespace_to_characters(true);
        cfg.ignore_end_of_stream = true;
        let mut reader = cfg.create_reader(Buffer::with_capacity(4096));

        reader.source_mut().write(INITIAL_STREAM.as_bytes()).unwrap();
        reader.source_mut().write(EXPECTED_MECHANISMS.as_bytes()).unwrap();

        let _ = reader.next().unwrap();
        let _ = reader.next().unwrap();
        let x = reader.next().unwrap();

        assert!(matches!(x, XmlEvent::StartElement { .. }));

        if let XmlEvent::StartElement { name, attributes, namespace } = x {
            let packet = Packet::parse(&mut reader, name, namespace, attributes);
            assert!(
                matches!(packet, Some(Packet::NonStanza(ref stanza)) if matches!(**stanza, NonStanza::StreamFeatures(StreamFeatures { features: Features::Mechanisms(ref m) }) if m == &mechs)),
                "Packet wasn't an StreamFeatures::Mechanisms, it was: {:?}",
                packet
            );
        }
    }
}
