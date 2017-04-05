use jid::Jid;
use std::str::FromStr;
use std::string::ParseError;
use std::str;

#[derive(Debug, Clone)]
pub struct Ping {
    from: Jid,
    to: String
}

impl Ping {
    pub fn new(from: Jid, to: &str) -> Ping {
        Ping {
            from: from,
            to: to.to_string()
        }
    }
}

impl FromStr for Ping {
    type Err = ParseError;
    fn from_str(_: &str) -> Result<Self, Self::Err> {
        Ok(Ping {
            from: Jid::from_str("").unwrap(),
            to: String::new()
        })
    }
}

impl ToString for Ping {
    fn to_string(&self) -> String {
        format!("<iq from='{from}' to='{to}' id='c2s1' type='get'>
                <ping xmlns='urn:xmpp:ping'/>
                </iq>", from=self.from.to_string(), to=self.to)
    }
}
