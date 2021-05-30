use crate::{ns, NonStanza, Packet, PacketParsingError, ToXmlElement};
use uuid::Uuid;
use xmpp_xml::{xml::attribute::OwnedAttribute, Element, QName};

/// Define an OpenStream NonStanza packet.
///
/// Mostly used when negociating connection between parties.
#[derive(derive_builder::Builder, Default, Debug, Clone)]
#[builder(setter(into))]
pub struct OpenStream {
    /// An Id generated by the server.
    ///
    /// The 'id' attribute specifies a unique identifier for the stream, called a "stream ID". The stream ID MUST be generated by the receiving entity when it sends a response stream header and MUST BE unique within the receiving application (normally a server).
    ///
    /// See [RFC-6120]( https://xmpp.org/rfcs/rfc6120.html#streams-attr-id )
    pub id: Uuid,
    /// The 'xml:lang' attribute specifies an entity's preferred or default language for any human-readable XML character data to be sent over the stream
    ///
    /// See [RFC-6120]( https://xmpp.org/rfcs/rfc6120.html#streams-attr-xmllang )
    pub lang: String,
    /// The inclusion of the version attribute set to a value of at least "1.0" signals support for the stream-related protocols defined in this specification, including TLS negotiation, SASL negotiation, stream features, and stream errors.
    ///
    /// See [RFC-6120]( https://xmpp.org/rfcs/rfc6120.html#streams-attr-version )
    pub version: String,
    #[builder(setter(into, strip_option), default)]
    pub to: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub from: Option<String>,
}

impl OpenStream {
    pub fn from_start_element(attributes: Vec<OwnedAttribute>) -> Result<Self, PacketParsingError> {
        let (to, lang, version) = attributes.iter().fold((String::from(""), String::from("en"), String::from("0.0")), |(to, lang, version), attribute| {
            match attribute.name.local_name.as_ref() {
                "to" if attribute.name.namespace.is_none() => (attribute.value.to_string(), lang, version),
                "lang" if attribute.name.namespace == Some(ns::XML_URI.to_string()) => (to, attribute.value.to_string(), version),
                "version" if attribute.name.namespace.is_none() => (to, lang, attribute.value.to_string()),
                _ => (to, lang, version),
            }
        });

        OpenStreamBuilder::default()
            .id(Uuid::new_v4())
            .to(to)
            .lang(lang)
            .version(version)
            .build()
            .or(Err(PacketParsingError::Unknown))
    }
}

impl From<OpenStream> for Packet {
    fn from(s: OpenStream) -> Self {
        NonStanza::OpenStream(s).into()
    }
}

impl ToXmlElement for OpenStream {
    type Error = std::io::Error;
    fn to_element(&self) -> Result<Element, Self::Error> {
        let mut element = Element::new((ns::CLIENT, "stream")).write_end_tag(false);

        let _ = element.set_namespace_prefix(ns::STREAM, "stream");

        let qname_stream = QName::from_ns_name(Some(ns::STREAM), "stream");
        element.set_tag(&qname_stream);
        element.set_attr("version", self.version.to_string());
        element.set_attr((ns::XML_URI, "lang"), self.lang.to_string());
        element.set_attr("id", self.id.to_string());

        if let Some(ref from) = self.from {
            element.set_attr("from", from.to_string());
        }

        if let Some(ref to) = self.to {
            element.set_attr("to", to.to_string());
        }

        Ok(element)
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

    #[test]
    fn from() {
        let initial_stream = "<stream:stream version='1.0' xmlns:stream='http://etherx.jabber.org/streams' to='example.com' xmlns='jabber:client'>";

        let mut cfg = ParserConfig::new().whitespace_to_characters(true);
        cfg.ignore_end_of_stream = true;
        let mut reader = cfg.create_reader(Buffer::with_capacity(4096));

        reader.source_mut().write(initial_stream.as_bytes()).unwrap();
        let _ = reader.next().unwrap();
        let x = reader.next().unwrap();

        assert!(matches!(x, XmlEvent::StartElement { .. }));

        if let XmlEvent::StartElement { name, attributes, namespace } = x {
            let packet = Packet::parse(&mut reader, name, namespace, attributes);
            assert!(
                matches!(packet, Ok(Packet::NonStanza(ref stanza)) if matches!(**stanza, NonStanza::OpenStream(_))),
                "Packet wasn't an OpenStream, it was: {:?}",
                packet
            );
        }
    }

    #[test]
    fn to_element() {
        let id = Uuid::new_v4();
        let lang: String = "en".into();
        let version: String = "1.0".into();
        let to: String = "jid@localhost".into();
        let from: String = "localhost".into();
        let expected = format!(
            r#"<stream:stream xmlns="jabber:client" xmlns:stream="http://etherx.jabber.org/streams" from="{from}" id="{id}" xml:lang="{lang}" to="{to}" version="{version}">"#,
            id = id,
            lang = lang,
            from = from,
            to = to,
            version = version
        );

        let open_stream = OpenStreamBuilder::default().id(id).lang(lang).version(version).to(to).from(from).build();

        assert!(open_stream.is_ok());

        assert!(matches!(open_stream, Ok(OpenStream { .. })));

        let mut output: Vec<u8> = Vec::new();
        let _ = open_stream.unwrap().to_element().unwrap().to_writer_with_options(&mut output, WriteOptions::new().set_xml_prolog(None));

        let generated = String::from_utf8(output).unwrap();

        assert!(expected == generated);
    }
}