use std::{cmp::Ordering, fmt, ops::Deref};

use string_cache::DefaultAtom;

pub enum XmlAtom<'a> {
    Shared(DefaultAtom),
    Borrowed(&'a str),
}

impl<'a> Deref for XmlAtom<'a> {
    type Target = str;

    #[inline(always)]
    fn deref(&self) -> &str {
        match *self {
            XmlAtom::Shared(ref atom) => atom.deref(),
            XmlAtom::Borrowed(s) => s,
        }
    }
}

impl<'a> XmlAtom<'a> {
    #[inline(always)]
    pub fn borrow(&self) -> &str {
        &self
    }
}

impl<'a> fmt::Debug for XmlAtom<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.borrow())
    }
}

impl<'a> Clone for XmlAtom<'a> {
    fn clone(&self) -> XmlAtom<'a> {
        XmlAtom::Shared(DefaultAtom::from(self.borrow()))
    }
}

impl<'a> PartialEq for XmlAtom<'a> {
    fn eq(&self, other: &XmlAtom<'a>) -> bool {
        self.borrow().eq(other.borrow())
    }
}

impl<'a> Eq for XmlAtom<'a> {}

impl<'a> PartialOrd for XmlAtom<'a> {
    fn partial_cmp(&self, other: &XmlAtom<'a>) -> Option<Ordering> {
        self.borrow().partial_cmp(other.borrow())
    }
}

impl<'a> Ord for XmlAtom<'a> {
    fn cmp(&self, other: &XmlAtom<'a>) -> Ordering {
        self.borrow().cmp(other.borrow())
    }
}
