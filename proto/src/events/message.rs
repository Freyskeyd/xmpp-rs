use std::str::FromStr;
use std::string::ParseError;
use std::str;

#[derive(Debug, Clone)]
pub struct Message {
    from: String,
    to: String,
    message_type: String,
    body: String,
    pub msg: String
}

impl Message {
    pub fn new(from: &str, to: &str) -> Message {
        Message {
            msg: to.to_string(),
            from: from.to_string(),
            to: to.to_string(),
            message_type: String::from("chat"),
            body: String::from("heyy"),
        }
    }
}

impl FromStr for Message {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Message {
            msg: String::from_str(s).unwrap(),
            from: String::new(),
            to: String::new(),
            message_type: String::new(),
            body: String::new(),
        })
    }
}

impl ToString for Message {
    fn to_string(&self) -> String {
        format!("<message to='{to}' from='{from}' type='{message_type}' id='purple6d50c1ba'><body>{body}</body></message>", to=self.to, from=self.from, message_type=self.message_type, body=self.body)
    }
}
