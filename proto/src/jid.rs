use std::result::Result;
use std::str::FromStr;
use std::string::ParseError;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub struct Jid {
    node: Option<String>,
    domain: String,
    resource: Option<String>
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
            resource: resource
        }
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
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, ParseError> {
        Ok(Jid {
            node: None,
            domain: input.to_string(),
            resource: None
        })
    }
}
// impl ToString for Jid {
//     fn to_string(&self) -> String {
//         let node = match self.node {
//             Some(s) => format!("{}@", s),
//             None => String::new()
//         };

//         let resource = match self.resource {
//             Some(r) => format!("/{}", r),
//             None => String::new()
//         };

//         format!("{node}{domain}{resource}", node=node, domain=self.domain, resource=resource)
//     }
// }
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

        assert!(Some("tt".to_string()) == jid.node, format!("{:?} == {:?}", Some("tt".to_string()), jid.node));
        assert!("zz.com".to_string() == jid.domain, format!("{:?} == {:?}", "zz.com".to_string(), jid.domain));
        assert!(Some("xx".to_string()) == jid.resource, format!("{:?} == {:?}", Some("xx".to_string()), jid.resource));
    }
}
