use std::str::FromStr;
use events::*;
use jid::{Jid, ToJid};
use super::Event;
use super::EventTrait;
use std::str;
use std::io;

#[derive(Debug, Clone, XmppEvent)]
#[stanza(is="message")]
pub struct Message {
    generic: GenericMessage,
    message_type: String,
    pub body: String
}

impl Message {
    pub fn new<T: ToJid + ?Sized, S: ToString>(to: &T, msg: &S) -> Message {
        Message {
            generic: GenericMessage::new(to),
            message_type: String::from("chat"),
            body: msg.to_string(),
        }
    }
}

impl FromStr for Message {
    type Err = io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // return Err(io::Error::new(io::ErrorKind::InvalidInput, ""))
        // let root = match Element::from_reader(s.as_bytes()) {
        //     Ok(r) => r,
        //     Err(e) => return Err(io::Error::new(io::ErrorKind::InvalidInput, e))
        // };


        // // `id` is REQUIRED
        // let id = match root.get_attr("id") {
        //     Some(id) => id.to_string(),
        //     None => return Err(io::Error::new(io::ErrorKind::InvalidInput, "ID is required"))
        // };

        // let iq_type = match IqType::from_str(root.get_attr("type").unwrap_or("")) {
        //     Ok(t) => t,
        //     Err(e) => return Err(e)
        // };

        // let from = match Jid::from_str(root.get_attr("from").unwrap_or("")) {
        //     Ok(j) => Some(j),
        //     Err(_) => None
        // };

        let generic = GenericMessage::from_str(s).unwrap();

        Ok(Message {
            generic: generic,
            message_type: String::new(),
            body: String::new(),
        })
    }
}

impl ToString for Message {
    fn to_string(&self) -> String {
        format!("<message to='{to}' from='{from}' type='{message_type}' id='purple6d50c1ba'><body>{body}</body></message>", to=self.get_to(), from=self.get_from().unwrap(), message_type=self.message_type, body=self.body)
    }
}
