use regex::Regex;
use regex::RegexSet;
use std::ops::Fn;

#[derive(Debug)]
pub struct RawStanza {
    pub inner: String
}

#[derive(Debug)]
pub enum NonStanzaType {
    StreamOpen(RawStanza),
    StreamFeatures(RawStanza),
    ProceedTls(RawStanza),
    SuccessTls(RawStanza),
}

#[derive(Debug)]
pub enum StanzaType {
    Message,
    Iq(RawStanza)
}

#[derive(Debug)]
pub enum MessageType {
    Unknown(RawStanza),
    NonStanza(NonStanzaType),
    Stanza(StanzaType)
}
impl MessageType {
    pub fn parse(f: &str) -> Vec<MessageType> {
        let matches = SET.matches(f);

        let mut v = Vec::new();
        for i in matches.into_iter() {
            match HASHMAP_R.get(i) {
                Some(s) => {
                    let c = s.1.captures(f).unwrap();
                    let ref cl = s.2;
                    v.push(cl(&c[0]));
                },
                None => {}
            }
        }

        v
    }
}

lazy_static! {
    static ref XML_R: &'static str = r"(<\?xm[^<]*>)";
    static ref STREAM_STREAM: &'static str = r"(<stream:stream[^<]*)";
    static ref STREAM_FEATURES: &'static str = r"(?i)(<stream:features>(.*?)(?:</stream:features>))";
    static ref PROCEED: &'static str = r"(<proceed[^<]*)";
    static ref SUCCESS: &'static str = r"(<success[^<]*)";
    static ref IQ: &'static str = r"(?i)(<iq(.*?)(?:</iq>))";

    static ref HASHMAP_R: Vec<(&'static str, Regex, Box<Fn(&str) -> MessageType + Sync>)> = {
        let mut m = Vec::new();

        m.push((*XML_R,
                Regex::new(&XML_R).unwrap(),
                Box::new(|c:&str| {
                    MessageType::Unknown(RawStanza {inner: c.to_string()})
                })
                as Box<Fn(&str) -> MessageType + Sync>
                ));

        m.push((*STREAM_STREAM,
                Regex::new(&STREAM_STREAM).unwrap(),
                Box::new(|c:&str| {
                    MessageType::NonStanza(NonStanzaType::StreamOpen(RawStanza {inner: c.to_string()}))
                })
                as Box<Fn(&str) -> MessageType + Sync>
                ));

        m.push((*STREAM_FEATURES,
                Regex::new(&STREAM_FEATURES).unwrap(),
                Box::new(|c:&str| {
                    MessageType::NonStanza(NonStanzaType::StreamFeatures(RawStanza {inner: c.to_string()}))
                })
                as Box<Fn(&str) -> MessageType + Sync>
                ));

        m.push((*PROCEED,
                Regex::new(&PROCEED).unwrap(),
                Box::new(|c:&str| {
                    MessageType::NonStanza(NonStanzaType::ProceedTls(RawStanza {inner: c.to_string()}))
                })
                as Box<Fn(&str) -> MessageType + Sync>
                ));

        m.push((*SUCCESS,
                Regex::new(&SUCCESS).unwrap(),
                Box::new(|c:&str| {
                    MessageType::NonStanza(NonStanzaType::SuccessTls(RawStanza {inner: c.to_string()}))
                })
                as Box<Fn(&str) -> MessageType + Sync>
                ));

        m.push((*IQ,
                Regex::new(&IQ).unwrap(),
                Box::new(|c:&str| {
                    MessageType::Stanza(StanzaType::Iq(RawStanza {inner: c.to_string()}))
                })
                as Box<Fn(&str) -> MessageType + Sync>
                ));

        m
    };

    static ref SET: RegexSet = RegexSet::new({ let mut v = Vec::new(); let ref z = *HASHMAP_R; for i in z.into_iter() { v.push(i.0); } v}).unwrap();
}
