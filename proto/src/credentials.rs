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
            password: "guest".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_default_values() {
        let c = Credentials { ..Credentials::default() };

        assert_eq!(c.jid, Jid::from_str("guest").unwrap());
        assert_eq!(c.password, "guest");
    }

    #[test]
    fn creation() {
        let c = Credentials { jid: Jid::from_str("guest").unwrap(), password: "guest".into() };
        assert_eq!(c.jid, Jid::from_str("guest").unwrap());
        assert_eq!(c.password, "guest");
    }

    #[test]
    fn equality() {
        let c = Credentials { jid: Jid::from_str("guest").unwrap(), password: "guest".into() };
        let mut d = c.clone();

        assert!(c == d);

        d.jid = Jid::from_str("guest2").unwrap();
        d.password = "guest".into();

        assert!(c != d);
    }
}
