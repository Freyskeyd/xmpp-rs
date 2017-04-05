use elementtree::Element;
use std::str::FromStr;
use std::string::ParseError;

#[derive(Debug, Clone)]
pub struct Bind {
    id: String,
    bind_type: String,
    pub body: Option<Element>,
    pub jid: String
}

impl Bind {
    pub fn new() -> Bind {
        Bind {
            id: String::new(),
            bind_type: String::new(),
            jid: String::new(),
            body: None
        }
    }

    pub fn set_type(mut self, t: &str) -> Self {
        self.bind_type = t.to_string();

        self
    }

    pub fn set_id(mut self, id: &str) -> Self {
        self.id = id.to_string();

        self
    }
}

impl FromStr for Bind {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let root = Element::from_reader(s.as_bytes()).unwrap();

        let bind_type = root.get_attr("type").unwrap_or("").to_string();
        let id = root.get_attr("id").unwrap_or("").to_string();
        let jid = match root.find("jid") {
            Some(jid) => jid.text().to_string(),
            None => String::new()
        };

        Ok(Bind {
            bind_type: bind_type,
            id: id,
            jid: jid,
            body: Some(root)
        })
    }
}

impl ToString for Bind {
    fn to_string(&self) -> String {
        format!("<iq type='{bind_type}' id='{id}'><bind xmlns='urn:ietf:params:xml:ns:xmpp-bind'/></iq>", id=self.id, bind_type=self.bind_type)
    }
}


#[derive(Debug, Clone)]
pub struct Generic {
    pub id: String,
    pub iq_type: String,
    pub body: Option<Element>
}

impl Generic {
    pub fn new() -> Generic {
        Generic {
            id: String::new(),
            iq_type: String::new(),
            body: None
        }
    }

    pub fn set_type(mut self, t: &str) -> Self {
        self.iq_type = t.to_string();

        self
    }

    pub fn set_id(mut self, id: &str) -> Self {
        self.id = id.to_string();

        self
    }
}

impl FromStr for Generic {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let root = Element::from_reader(s.as_bytes()).unwrap();

        let iq_type = root.get_attr("type").unwrap_or("").to_string();
        let id = root.get_attr("id").unwrap_or("").to_string();

        Ok(Generic {
            iq_type: iq_type,
            id: id,
            body: Some(root)
        })
    }
}

impl ToString for Generic {
    fn to_string(&self) -> String {
        String::new()
    }
}
