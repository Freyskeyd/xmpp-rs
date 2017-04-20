use regex::Regex;
use regex::RegexSet;
use ns;
use events::Event;
use events::Event::*;
use events::IqType;
use events::NonStanzaEvent::*;
use events::StanzaEvent::*;
use events::IqEvent::*;
// use events::Generic;
use events::*;

use std::ops::Fn;
use events::{OpenStream, EventTrait, Presence, Message,StreamFeatures, Unknown, SuccessTls, ProceedTls};
use std::str::FromStr;

/// Used to parse incoming stanza into Event
///
/// # Examples
/// ```
/// use xmpp_proto::Parser;
///
/// match Parser::parse("<proceed xmlns='' xmlns:stream=''/>") {
///     Some(_) => {}
///     None => panic!("")
/// };
/// ```
pub struct Parser;

impl Parser {
    /// Parse an incoming `stanza` and return matching event
    ///
    pub fn parse(f: &str) -> Option<(Event, String)> {
        let matches:Vec<_> = SET.matches(f).into_iter().collect();

        if !matches.is_empty() {
            match matches.first() {
                Some(i) => {
                    match HASHMAP_R.get(*i) {
                        Some(s) => {
                            let c = s.1.captures(f).unwrap();
                            let cl = &s.2;
                            Some((cl(&c[0]), c[0].to_string()))
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
    static ref STREAM_CLOSE: &'static str = r"(</stream:stream>)";
    static ref STREAM_FEATURES: &'static str = r"(?i)(<stream:features>(.*?)(?:</stream:features>))";
    static ref PROCEED: &'static str = r"(<proceed[^<]*)";
    static ref SUCCESS: &'static str = r"(<success[^<]*)";
    static ref IQ: &'static str = r"(?i)((<iq[^<]*/>|<iq(.*?)(?:</iq>)))";
    static ref PRESENCE: &'static str = r"(?i)((<presence[^<]*/>|<presence(.*?)(?:</presence>)))";
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
                    let o = OpenStream::from_str(c).unwrap();

                    o.to_event()
                })
                as Box<Fn(&str) -> Event + Sync>
                ));

        m.push((*STREAM_CLOSE,
                Regex::new(&STREAM_CLOSE).unwrap(),
                Box::new(|_:&str| {
                    let o = CloseStream {};
                    o.to_event()
                })
                as Box<Fn(&str) -> Event + Sync>
                ));

        m.push((*STREAM_FEATURES,
                Regex::new(&STREAM_FEATURES).unwrap(),
                Box::new(|c:&str| {
                    NonStanza(Box::new(StreamFeaturesEvent(StreamFeatures::from_str(c).unwrap())), c.to_string())
                })
                as Box<Fn(&str) -> Event + Sync>
                ));

        m.push((*PROCEED,
                Regex::new(&PROCEED).unwrap(),
                Box::new(|c:&str| {
                    NonStanza(Box::new(ProceedTlsEvent(ProceedTls::from_str(c).unwrap())), c.to_string())
                })
                as Box<Fn(&str) -> Event + Sync>
                ));

        m.push((*SUCCESS,
                Regex::new(&SUCCESS).unwrap(),
                Box::new(|c:&str| {
                    NonStanza(Box::new(SuccessTlsEvent(SuccessTls::from_str(c).unwrap())), c.to_string())
                })
                as Box<Fn(&str) -> Event + Sync>
                ));

        m.push((*IQ,
                Regex::new(&IQ).unwrap(),
                Box::new(|c:&str| {
                    let g = GenericIq::from_str(c).unwrap();
                    match g.get_type() {
                        IqType::Get => {
                            match g.get_element() {
                                Some(body) if body.find((ns::PING, "ping")).is_some() => {
                                    return Stanza(Box::new(IqRequestEvent(Box::new(PingEvent(Ping::from_str(c).unwrap())))), c.to_string())},
                                _ => {}
                            };
                            Stanza(Box::new(IqRequestEvent(Box::new(GenericEvent(g)))), c.to_string())
                        },
                        IqType::Set => g.to_event(),
                        IqType::Result => Stanza(Box::new(IqResponseEvent(Box::new(GenericEvent(g)))), c.to_string()),
                        IqType::Error => g.to_event()
                    }
                })
                as Box<Fn(&str) -> Event + Sync>
                ));

        m.push((*PRESENCE,
                Regex::new(&PRESENCE).unwrap(),
                Box::new(|c:&str| {
                    Stanza(Box::new(PresenceEvent(Presence::from_str(c).unwrap())), c.to_string())
                })
                as Box<Fn(&str) -> Event + Sync>
                ));

        m.push((*MESSAGE,
                Regex::new(&MESSAGE).unwrap(),
                Box::new(|c:&str| {
                    let o = Message::from_str(c).unwrap();
                    o.to_event()
                })
                as Box<Fn(&str) -> Event + Sync>
                ));

        m
    };

    static ref SET: RegexSet = RegexSet::new({ let mut v = Vec::new(); let z = &*HASHMAP_R; for i in z { v.push(i.0); } v}).unwrap();
}
