use regex::Regex;
use regex::RegexSet;
use events::Event;
use events::Event::*;
use events::NonStanzaEvent::*;
use events::StanzaEvent::*;
use events::IqType::*;
use events::Generic;

use std::ops::Fn;
use events::{OpenStream, Presence, Message,StreamFeatures, Unknown, SuccessTls, ProceedTls};
use std::str::FromStr;

pub struct Parser;

impl Parser {
    pub fn parse(f: &str) -> Option<Event> {
        let matches:Vec<_> = SET.matches(f).into_iter().collect();

        if !matches.is_empty() {
            match matches.first() {
                Some(i) => {
                    match HASHMAP_R.get(*i) {
                        Some(s) => {
                            let c = s.1.captures(f).unwrap();
                            let cl = &s.2;
                            Some(cl(&c[0]))
                        },
                        None => None
                    }
                },
                None => None
            }
        } else {
            None
        }
    }
}

lazy_static! {
    static ref XML_R: &'static str = r"(<\?xm[^<]*>)";
    static ref STREAM_STREAM: &'static str = r"(<stream:stream[^<]*)";
    static ref STREAM_FEATURES: &'static str = r"(?i)(<stream:features>(.*?)(?:</stream:features>))";
    static ref PROCEED: &'static str = r"(<proceed[^<]*)";
    static ref SUCCESS: &'static str = r"(<success[^<]*)";
    static ref IQ: &'static str = r"(?i)(<iq(.*?)(?:(</iq>|/>)))";
    static ref PRESENCE: &'static str = r"(?i)(<presence(.*?)(?:/>))";
    static ref MESSAGE: &'static str = r"(?i)(<message(.*?)(?:</message>))";

    static ref HASHMAP_R: Vec<(&'static str, Regex, Box<Fn(&str) -> Event + Sync>)> = {
        let mut m = Vec::new();

        m.push((*XML_R,
                Regex::new(&XML_R).unwrap(),
                Box::new(|c:&str| {
                    Unknown(Unknown::from_str(c).unwrap(), c.to_string())
                })
                as Box<Fn(&str) -> Event + Sync>
                ));

        m.push((*STREAM_STREAM,
                Regex::new(&STREAM_STREAM).unwrap(),
                Box::new(|c:&str| {
                    NonStanza(OpenStreamEvent(OpenStream::from_str(c).unwrap()), c.to_string())
                })
                as Box<Fn(&str) -> Event + Sync>
                ));

        m.push((*STREAM_FEATURES,
                Regex::new(&STREAM_FEATURES).unwrap(),
                Box::new(|c:&str| {
                    NonStanza(StreamFeaturesEvent(StreamFeatures::from_str(c).unwrap()), c.to_string())
                })
                as Box<Fn(&str) -> Event + Sync>
                ));

        m.push((*PROCEED,
                Regex::new(&PROCEED).unwrap(),
                Box::new(|c:&str| {
                    NonStanza(ProceedTlsEvent(ProceedTls::from_str(c).unwrap()), c.to_string())
                })
                as Box<Fn(&str) -> Event + Sync>
                ));

        m.push((*SUCCESS,
                Regex::new(&SUCCESS).unwrap(),
                Box::new(|c:&str| {
                    NonStanza(SuccessTlsEvent(SuccessTls::from_str(c).unwrap()), c.to_string())
                })
                as Box<Fn(&str) -> Event + Sync>
                ));

        m.push((*IQ,
                Regex::new(&IQ).unwrap(),
                Box::new(|c:&str| {
                    Stanza(IqEvent(GenericIq(Generic::from_str(c).unwrap())), c.to_string())
                })
                as Box<Fn(&str) -> Event + Sync>
                ));

        m.push((*PRESENCE,
                Regex::new(&PRESENCE).unwrap(),
                Box::new(|c:&str| {
                    Stanza(PresenceEvent(Presence::from_str(c).unwrap()), c.to_string())
                })
                as Box<Fn(&str) -> Event + Sync>
                ));

        m.push((*MESSAGE,
                Regex::new(&MESSAGE).unwrap(),
                Box::new(|c:&str| {
                    Stanza(MessageEvent(Message::from_str(c).unwrap()), c.to_string())
                })
                as Box<Fn(&str) -> Event + Sync>
                ));

        m
    };

    static ref SET: RegexSet = RegexSet::new({ let mut v = Vec::new(); let z = &*HASHMAP_R; for i in z { v.push(i.0); } v}).unwrap();
}
