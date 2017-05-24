use std::result::Result;
use std::str::FromStr;
use std::fmt;
use std::io;

///
/// Represents a Jabber ID (JID)
///
/// A JID is composed by two optionals components and a required one.
///
/// See [RFC-6122](https://tools.ietf.org/html/rfc6122)
///
#[derive(Clone, Debug, PartialEq)]
pub struct Jid {
    pub node: Option<String>,
    pub domain: String,
    pub resource: Option<String>,
}

impl Jid {
    pub fn from_full_jid(jid: &str) -> Jid {
        let mut node: Option<String> = None;
        let mut resource: Option<String> = None;
        let mut domain: String;

        let node_and_domain = jid.split('@').collect::<Vec<&str>>();
        if node_and_domain.len() > 1 {
            node = Some(node_and_domain[0].to_string());
            domain = node_and_domain[1].to_string();
        } else {
            domain = node_and_domain[0].to_string();
        }

        let d = domain.clone();
        let domain_and_resource = d.split('/').collect::<Vec<&str>>();
        domain = domain_and_resource[0].to_string();
        if domain_and_resource.len() > 1 {
            resource = Some(domain_and_resource[1].to_string());
        }

        Jid {
            node: node,
            domain: domain,
            resource: resource,
        }
    }
}

impl ToJid for Jid {
    fn to_jid(&self) -> Result<Jid, io::Error> {
        Ok(self.clone())
    }
}

impl fmt::Display for Jid {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        // TODO: may need escaping
        if let Some(ref node) = self.node {
            write!(fmt, "{}@", node)?;
        }
        write!(fmt, "{}", self.domain)?;
        if let Some(ref resource) = self.resource {
            write!(fmt, "/{}", resource)?;
        }
        Ok(())
    }
}
impl FromStr for Jid {
    type Err = io::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Jid::from_full_jid(input))
    }
}

pub trait ToJid {
    fn to_jid(&self) -> Result<Jid, io::Error>;
}

impl<'a> ToJid for &'a str {
    fn to_jid(&self) -> Result<Jid, io::Error> {
        Jid::from_str(self)
    }
}

impl ToJid for str {
    fn to_jid(&self) -> Result<Jid, io::Error> {
        Jid::from_str(self)
    }
}

impl<'a, T> ToJid for Option<&'a T>
    where T: ToJid + ?Sized
{
    fn to_jid(&self) -> Result<Jid, io::Error> {
        match *self {
            Some(t) => t.to_jid(),
            None => Err(io::Error::new(io::ErrorKind::InvalidInput, "")),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jid_no_node() {
        let jid = Jid::from_full_jid("zz.com/xx");

        assert!(None == jid.node);
        assert!("zz.com".to_string() == jid.domain);
        assert!(Some("xx".to_string()) == jid.resource);
    }

    #[test]
    fn jid_from_full() {
        let jid = Jid::from_full_jid("tt@zz.com/xx");

        assert!(Some("tt".to_string()) == jid.node,
                format!("{:?} == {:?}", Some("tt".to_string()), jid.node));
        assert!("zz.com".to_string() == jid.domain,
                format!("{:?} == {:?}", "zz.com".to_string(), jid.domain));
        assert!(Some("xx".to_string()) == jid.resource,
                format!("{:?} == {:?}", Some("xx".to_string()), jid.resource));
    }

    #[test]
    fn to_jid_fail() {
        let j = "x";

        match j.to_jid() {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
    }
}
