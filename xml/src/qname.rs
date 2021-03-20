use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::{borrow::Cow, cmp::Ord};

use string_cache::DefaultAtom as Atom;

use xml::name::OwnedName;

use crate::XmlAtom;

/// A `QName` represents a qualified name.
///
/// A qualified name is a tag or attribute name that has a namespace and a
/// local name.  If the namespace is empty no namespace is assumed.  It
/// can be constructed from a qualified name string with the ``from``
/// method.
///
/// ## Notes on Memory Management
///
/// Qualified names that are user constructed for comparison purposes
/// usually have a static lifetime because they are created from static
/// strings.  Creating qualified names from other strings might make
/// memory management harder which is why `share()` exists which moves
/// the `QName` internal strings to shared storage in which the lifetime
/// changes to `'static`.
///
/// Common usage examples:
///
/// ```no_run
/// # use xmpp_xml::QName;
/// let href = QName::from_name("href");
/// let a = QName::from("{http://www.w3.org/1999/xhtml}a");
/// ```
#[derive(Clone)]
pub struct QName<'a> {
    ns: Option<XmlAtom<'a>>,
    name: XmlAtom<'a>,
}

impl<'a> QName<'a> {
    /// Creates a qualified name from a given string.
    ///
    /// Two formats are supported ``{namespace}tag`` or just ``tag``.
    ///
    /// ```
    /// # use xmpp_xml::QName;
    /// let a = QName::from("{http://www.w3.org/1999/xhtml}a");
    /// ```
    pub fn from(s: &'a str) -> QName<'a> {
        let mut ns = None;
        let mut name = None;
        if s.starts_with('{') {
            if let Some(index) = s.find('}') {
                if index > 1 {
                    ns = Some(XmlAtom::Borrowed(&s[1..index]));
                }
                name = Some(XmlAtom::Borrowed(&s[index + 1..]));
            }
        }

        QName {
            ns,
            name: name.unwrap_or_else(|| XmlAtom::Borrowed(s)),
        }
    }

    /// Creates a qualified name from a given string without namespace.
    ///
    /// This is slightly faster than using ``from()``.
    pub fn from_name(name: &'a str) -> QName<'a> {
        QName {
            ns: None,
            name: XmlAtom::Borrowed(name),
        }
    }

    /// Creates a qualified name from a namespace and name.
    pub fn from_ns_name(ns: Option<&'a str>, name: &'a str) -> QName<'a> {
        QName {
            ns: ns.map(|x| XmlAtom::Borrowed(x)),
            name: XmlAtom::Borrowed(name),
        }
    }

    /// Returns the name portion of the qualified name.  This is the local
    /// tag or attribute name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the optional namespace of this element.  This is the URL of
    /// the namespace and not the prefix.  The information about the latter
    /// is not retained.
    pub fn ns(&self) -> Option<&str> {
        self.ns.as_ref().map(|x| x.borrow())
    }

    /// Creates a shared `QName` with static lifetime from an already
    /// existing `QName`.  The internal strings are interned and might
    /// be shared with other instances.
    pub fn share(&self) -> QName<'static> {
        QName {
            name: XmlAtom::Shared(Atom::from(self.name.borrow())),
            ns: self.ns.as_ref().map(|x| XmlAtom::Shared(Atom::from(x.borrow()))),
        }
    }

    pub(crate) fn from_owned_name(name: OwnedName) -> QName<'static> {
        QName {
            name: XmlAtom::Shared(Atom::from(name.local_name)),
            ns: match name.namespace {
                Some(ns) => {
                    if ns.len() > 0 {
                        Some(XmlAtom::Shared(Atom::from(ns)))
                    } else {
                        None
                    }
                }
                _ => None,
            },
        }
    }
}

impl<'a> PartialEq for QName<'a> {
    fn eq(&self, other: &QName<'a>) -> bool {
        self.name() == other.name() && self.ns() == other.ns()
    }
}

impl<'a> fmt::Debug for QName<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "QName(\"{}\")", self)
    }
}

impl<'a> fmt::Display for QName<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref ns) = self.ns {
            write!(f, "{{{}}}", ns.borrow())?;
        }
        write!(f, "{}", self.name.borrow())
    }
}

impl<'a> Eq for QName<'a> {}

impl<'a> Hash for QName<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        if let Some(ref ns) = self.ns {
            ns.hash(state);
        }
    }
}

impl<'a> PartialOrd for QName<'a> {
    fn partial_cmp(&self, other: &QName<'a>) -> Option<Ordering> {
        self.name().partial_cmp(other.name())
    }
}

impl<'a> Ord for QName<'a> {
    fn cmp(&self, other: &QName<'a>) -> Ordering {
        self.name().cmp(other.name())
    }
}

/// Convenience trait to get a `QName` from an object.
///
/// This is used for the accessor interface on elements.
pub trait AsQName<'a> {
    /// Returns a Cow'ed `QName` from the given object.
    fn as_qname(&self) -> Cow<'a, QName<'a>>;
}

impl<'a> AsQName<'a> for &'a QName<'a> {
    #[inline(always)]
    fn as_qname(&self) -> Cow<'a, QName<'a>> {
        Cow::Borrowed(self)
    }
}

impl<'a> AsQName<'a> for &'a str {
    #[inline(always)]
    fn as_qname(&self) -> Cow<'a, QName<'a>> {
        Cow::Owned(QName::from(self))
    }
}

impl<'a> AsQName<'a> for (&'a str, &'a str) {
    #[inline(always)]
    fn as_qname(&self) -> Cow<'a, QName<'a>> {
        Cow::Owned(QName::from_ns_name(Some(self.0), self.1))
    }
}
