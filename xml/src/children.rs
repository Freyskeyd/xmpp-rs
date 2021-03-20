use std::borrow::Cow;
use std::collections::btree_map::Iter as BTreeMapIter;

use crate::{Element, QName};

/// An iterator over children of an element.
pub struct Children<'a> {
    pub(crate) idx: usize,
    pub(crate) element: &'a Element,
}

/// A mutable iterator over children of an element.
pub struct ChildrenMut<'a> {
    pub(crate) iter: ::std::slice::IterMut<'a, Element>,
}

/// An iterator over attributes of an element.
pub struct Attrs<'a> {
    pub(crate) iter: BTreeMapIter<'a, QName<'a>, String>,
}

/// An iterator over matching children.
pub struct FindChildren<'a> {
    pub(crate) tag: Cow<'a, QName<'a>>,
    pub(crate) child_iter: Children<'a>,
}

/// A mutable iterator over matching children.
pub struct FindChildrenMut<'a> {
    pub(crate) tag: Cow<'a, QName<'a>>,
    pub(crate) child_iter: ChildrenMut<'a>,
}

impl<'a> Iterator for Children<'a> {
    type Item = &'a Element;

    fn next(&mut self) -> Option<&'a Element> {
        if self.idx < self.element.children.len() {
            let rv = &self.element.children[self.idx];
            self.idx += 1;
            Some(rv)
        } else {
            None
        }
    }
}

impl<'a> Iterator for ChildrenMut<'a> {
    type Item = &'a mut Element;

    fn next(&mut self) -> Option<&'a mut Element> {
        self.iter.next()
    }
}

impl<'a> Iterator for FindChildren<'a> {
    type Item = &'a Element;

    fn next(&mut self) -> Option<&'a Element> {
        use std::borrow::Borrow;
        loop {
            if let Some(child) = self.child_iter.next() {
                if child.tag() == self.tag.borrow() {
                    return Some(child);
                }
            } else {
                return None;
            }
        }
    }
}

impl<'a> Iterator for FindChildrenMut<'a> {
    type Item = &'a mut Element;

    fn next(&mut self) -> Option<&'a mut Element> {
        use std::borrow::Borrow;
        let tag: &QName = &self.tag.borrow();
        self.child_iter.find(|x| x.tag() == tag)
    }
}

impl<'a> Iterator for Attrs<'a> {
    type Item = (&'a QName<'a>, &'a str);

    fn next(&mut self) -> Option<(&'a QName<'a>, &'a str)> {
        if let Some((k, v)) = self.iter.next() {
            Some((k, v.as_str()))
        } else {
            None
        }
    }
}
