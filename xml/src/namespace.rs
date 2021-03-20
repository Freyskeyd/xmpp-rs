use std::{collections::BTreeMap, mem};

use string_cache::Atom;

use crate::{Error, XmlAtom};

#[derive(Debug, Clone)]
pub struct NamespaceMap {
    pub(crate) prefix_to_ns: BTreeMap<XmlAtom<'static>, XmlAtom<'static>>,
    ns_to_prefix: BTreeMap<XmlAtom<'static>, XmlAtom<'static>>,
}

impl NamespaceMap {
    pub fn new() -> NamespaceMap {
        NamespaceMap {
            prefix_to_ns: BTreeMap::new(),
            ns_to_prefix: BTreeMap::new(),
        }
    }

    pub fn get_prefix(&self, url: &str) -> Option<&str> {
        // same shit as with Element::remove_attr for the explanation.
        let atom = XmlAtom::Borrowed(url);
        let static_atom: &XmlAtom<'static> = unsafe { mem::transmute(&atom) };
        self.ns_to_prefix.get(static_atom).map(|x| x.borrow())
    }

    pub fn set_prefix(&mut self, url: &str, prefix: &str) -> Result<(), Error> {
        let prefix = XmlAtom::Shared(Atom::from(prefix));
        if self.prefix_to_ns.contains_key(&prefix) {
            return Err(Error::DuplicateNamespacePrefix);
        }

        let url = XmlAtom::Shared(Atom::from(url));
        if let Some(old_prefix) = self.ns_to_prefix.remove(&url) {
            self.prefix_to_ns.remove(&old_prefix);
        }

        self.ns_to_prefix.insert(url.clone(), prefix.clone());
        self.prefix_to_ns.insert(prefix.clone(), url.clone());

        Ok(())
    }

    fn generate_prefix(&self) -> XmlAtom<'static> {
        let mut i = 1;
        loop {
            let random_prefix = format!("ns{}", i);
            if !self.prefix_to_ns.contains_key(&XmlAtom::Borrowed(&random_prefix)) {
                return XmlAtom::Shared(Atom::from(random_prefix));
            }
            i += 1;
        }
    }

    pub fn register_if_missing(&mut self, url: &str, prefix: Option<&str>) -> bool {
        if self.get_prefix(url).is_some() {
            return false;
        }

        let stored_prefix = if let Some(prefix) = prefix {
            let prefix = XmlAtom::Borrowed(prefix);
            if self.prefix_to_ns.get(&prefix).is_some() {
                self.generate_prefix()
            } else {
                XmlAtom::Shared(Atom::from(prefix.borrow()))
            }
        } else {
            self.generate_prefix()
        };

        let url = XmlAtom::Shared(Atom::from(url));
        self.prefix_to_ns.insert(stored_prefix.clone(), url.clone());
        self.ns_to_prefix.insert(url, stored_prefix);
        true
    }
}
