use jid::Jid;
use std::str::FromStr;

#[derive(Clone,Debug,PartialEq)]
pub struct Credentials {
    pub jid: Jid,
    pub password: String,
}

impl Default for Credentials {
    fn default() -> Credentials {
        Credentials {
            jid: Jid::from_str("guest").unwrap(),
            password: "guest".to_string(),
        }
    }
}
