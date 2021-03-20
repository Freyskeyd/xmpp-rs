use std::borrow::Cow;
use std::collections::BTreeMap;
use std::io::{Read, Write};
use std::mem;
use std::rc::Rc;

use xml::attribute::{Attribute, OwnedAttribute};
use xml::common::XmlVersion;
use xml::name::{Name, OwnedName};
use xml::namespace::{Namespace as XmlNamespaceMap, NS_EMPTY_URI, NS_XMLNS_URI, NS_XML_URI};
use xml::reader::{EventReader, ParserConfig, XmlEvent};
use xml::writer::{EventWriter, XmlEvent as XmlWriteEvent};
use xml::EmitterConfig;

use crate::{AsQName, Attrs, Children, ChildrenMut, Error, FindChildren, FindChildrenMut, NamespaceMap, Position, QName, WriteOptions, XmlProlog};

/// Represents an XML element.
///
/// Usually constructed from either parsing or one of the two constructors
/// an element is part of a tree and represents an XML element and the
/// children contained.
///
/// Imagine a structure like this:
///
/// ```xml
/// <p>Hello <strong>World</strong>!</p>
/// ```
///
/// In this case the structure is more or less represented like this:
///
/// ```ignore
/// Element {
///   tag: "p",
///   text: "Hello ",
///   tail: None,
///   children: [
///     Element {
///       tag: "strong",
///       text: "World",
///       tail: Some("!")
///     }
///   ]
/// }
/// ```
///
/// Namespaces are internally managed and inherited downwards when an
/// element is created.
#[derive(Debug, Clone)]
pub struct Element {
    tag: QName<'static>,
    attributes: BTreeMap<QName<'static>, String>,
    pub(crate) children: Vec<Element>,
    nsmap: Option<Rc<NamespaceMap>>,
    emit_nsmap: bool,
    text: Option<String>,
    tail: Option<String>,
    write_end_tag: bool,
}
impl Element {
    /// Creates a new element without any children but a given tag.
    ///
    /// This can be used at all times to create a new element however when you
    /// work with namespaces it's recommended to only use this for the root
    /// element and then create further children through `new_with_namespaces`
    /// as otherwise namespaces will not be propagaged downwards properly.
    pub fn new<'a, Q: AsQName<'a>>(tag: Q) -> Element {
        Element::new_with_nsmap(&tag.as_qname(), None)
    }

    /// Creates a new element without any children but inheriting the
    /// namespaces from another element.
    ///
    /// This has the advantage that internally the map will be shared
    /// across elements for as long as no further modifications are
    /// taking place.
    pub fn new_with_namespaces<'a, Q: AsQName<'a>>(tag: Q, reference: &Element) -> Element {
        Element::new_with_nsmap(&tag.as_qname(), reference.nsmap.clone())
    }

    fn new_with_nsmap<'a>(tag: &QName<'a>, nsmap: Option<Rc<NamespaceMap>>) -> Element {
        let mut rv = Element {
            tag: tag.share(),
            attributes: BTreeMap::new(),
            nsmap,
            emit_nsmap: false,
            children: vec![],
            text: None,
            tail: None,
            write_end_tag: true,
        };
        if let Some(url) = tag.ns() {
            let prefix = rv.get_namespace_prefix(url).unwrap_or("").to_string();
            rv.register_namespace(url, Some(&prefix));
        }
        rv
    }

    /// Parses some XML data into an `Element` from a reader.
    pub fn from_reader<R: Read>(r: R) -> Result<Element, Error> {
        let cfg = ParserConfig::new().whitespace_to_characters(true);
        let mut reader = cfg.create_reader(r);
        loop {
            match reader.next() {
                Ok(XmlEvent::StartElement { name, attributes, namespace }) => {
                    return Element::from_start_element(name, attributes, namespace, None, &mut reader);
                }
                Ok(XmlEvent::Comment(..)) | Ok(XmlEvent::Whitespace(..)) | Ok(XmlEvent::StartDocument { .. }) | Ok(XmlEvent::ProcessingInstruction { .. }) => {
                    continue;
                }
                Ok(_) => {
                    return Err(Error::UnexpectedEvent {
                        msg: Cow::Borrowed("xml construct"),
                        pos: Position::from_xml_position(&reader),
                    })
                }
                Err(e) => return Err(e.into()),
            }
        }
    }

    /// Dump an element as XML document into a writer.
    ///
    /// This will create an XML document with a processing instruction
    /// to start it.  There is currently no API to only serialize a non
    /// standalone element.
    ///
    /// Currently the writer has no way to customize what is generated
    /// in particular there is no support yet for automatically indenting
    /// elements.  The reason for this is that there is no way to ignore
    /// this information automatically in the absence of DTD support which
    /// is not really planned.
    pub fn to_writer<W: Write>(&self, w: W) -> Result<(), Error> {
        self.to_writer_with_options(w, WriteOptions::new())
    }

    /// Dump an element as XML document into a writer with option.
    ///
    /// This will create an XML document with a processing instruction
    /// to start it.  There is currently no API to only serialize a non
    /// standalone element.
    ///
    /// Currently the writer has no way to customize what is generated
    /// in particular there is no support yet for automatically indenting
    /// elements.  The reason for this is that there is no way to ignore
    /// this information automatically in the absence of DTD support which
    /// is not really planned.
    pub fn to_writer_with_options<W: Write>(&self, w: W, options: WriteOptions) -> Result<(), Error> {
        let mut writer = EmitterConfig::new()
            .normalize_empty_elements(self.write_end_tag)
            .write_document_declaration(options.xml_prolog.is_some())
            .create_writer(w);

        if options.xml_prolog.is_some() {
            writer.write(XmlWriteEvent::StartDocument {
                version: match options.xml_prolog.unwrap() {
                    XmlProlog::Version10 => XmlVersion::Version10,
                    XmlProlog::Version11 => XmlVersion::Version11,
                },
                encoding: Some("utf-8"),
                standalone: None,
            })?;
        }

        self.dump_into_writer(&mut writer)
    }

    /// Dump an element as XML document into a string
    pub fn to_string(&self) -> Result<String, Error> {
        let mut out: Vec<u8> = Vec::new();
        self.to_writer(&mut out)?;
        Ok(String::from_utf8(out).unwrap())
    }

    fn get_xml_name<'a>(&'a self, qname: &'a QName<'a>) -> Name<'a> {
        let mut name = Name::local(qname.name());
        if let Some(url) = qname.ns() {
            name.namespace = Some(url);
            if let Some(prefix) = self.get_namespace_prefix(url) {
                if !prefix.is_empty() {
                    name.prefix = Some(prefix);
                }
            }
        }
        name
    }

    fn dump_into_writer<W: Write>(&self, w: &mut EventWriter<W>) -> Result<(), Error> {
        let name = self.get_xml_name(&self.tag);

        let mut attributes = Vec::with_capacity(self.attributes.len());
        for (k, v) in self.attributes.iter() {
            attributes.push(Attribute { name: self.get_xml_name(k), value: v });
        }

        let mut namespace = XmlNamespaceMap::empty();
        if self.emit_nsmap {
            if let Some(ref nsmap) = self.nsmap {
                for (prefix, url) in &nsmap.prefix_to_ns {
                    namespace.put(prefix.borrow(), url.borrow());
                }
            }
        }

        w.write(XmlWriteEvent::StartElement {
            name,
            attributes: Cow::Owned(attributes),
            namespace: Cow::Owned(namespace),
        })?;

        let text = self.text();
        if !text.is_empty() {
            w.write(XmlWriteEvent::Characters(text))?;
        }

        for elem in &self.children {
            elem.dump_into_writer(w)?;
            let text = elem.tail();
            if !text.is_empty() {
                w.write(XmlWriteEvent::Characters(text))?;
            }
        }

        if self.write_end_tag {
            w.write(XmlWriteEvent::EndElement { name: Some(name) })?;
        }

        Ok(())
    }

    pub fn from_xml_start_element<R: Read>(start_element: &xml::reader::XmlEvent, reader: &mut EventReader<R>) -> Result<Self, Error> {
        match start_element {
            XmlEvent::StartElement { name, attributes, namespace } => Self::from_start_element(name.to_owned(), attributes.to_owned(), namespace.to_owned(), None, reader),
            _ => Err(Error::DuplicateNamespacePrefix),
        }
    }
    pub fn from_start_element<R: Read>(
        name: OwnedName,
        attributes: Vec<OwnedAttribute>,
        namespace: XmlNamespaceMap,
        parent_nsmap: Option<Rc<NamespaceMap>>,
        reader: &mut EventReader<R>,
    ) -> Result<Element, Error> {
        let mut root = Element {
            tag: QName::from_owned_name(name),
            attributes: BTreeMap::new(),
            nsmap: parent_nsmap,
            emit_nsmap: false,
            children: vec![],
            text: None,
            tail: None,
            write_end_tag: true,
        };
        for attr in attributes {
            root.attributes.insert(QName::from_owned_name(attr.name), attr.value);
        }

        if !namespace.is_essentially_empty() {
            for (prefix, url) in namespace.0.iter() {
                root.register_namespace(url, Some(prefix));
            }
        };

        root.parse_children(reader)?;
        Ok(root)
    }

    fn parse_children<R: Read>(&mut self, reader: &mut EventReader<R>) -> Result<(), Error> {
        loop {
            match reader.next() {
                Ok(XmlEvent::EndElement { ref name }) => {
                    if &name.local_name == self.tag.name() && name.namespace.as_ref().map(|x| x.as_str()) == self.tag.ns() {
                        return Ok(());
                    } else {
                        return Err(Error::UnexpectedEvent {
                            msg: Cow::Owned(format!("Unexpected end element {}", &name.local_name)),
                            pos: Position::from_xml_position(reader),
                        });
                    }
                }
                Ok(XmlEvent::StartElement { name, attributes, namespace }) => {
                    self.children.push(Element::from_start_element(name, attributes, namespace, self.nsmap.clone(), reader)?);
                }
                Ok(XmlEvent::Characters(s)) => {
                    let child_count = self.children.len();
                    if child_count > 0 {
                        self.children[child_count - 1].tail = Some(s);
                    } else {
                        self.text = Some(s);
                    }
                }
                Ok(XmlEvent::CData(s)) => {
                    self.text = Some(s);
                }
                Ok(XmlEvent::Comment(..)) | Ok(XmlEvent::Whitespace(..)) | Ok(XmlEvent::StartDocument { .. }) | Ok(XmlEvent::ProcessingInstruction { .. }) => {
                    continue;
                }
                Ok(_) => {
                    return Err(Error::UnexpectedEvent {
                        msg: Cow::Borrowed("unknown element"),
                        pos: Position::from_xml_position(reader),
                    })
                }
                Err(e) => {
                    return Err(e.into());
                }
            }
        }
    }

    /// Returns the text of a tag.
    ///
    /// Note that this does not trim or modify whitespace so the return
    /// value might contain structural information from the XML file.
    pub fn text(&self) -> &str {
        self.text.as_ref().map(|x| x.as_str()).unwrap_or("")
    }

    /// Sets a new text value for the tag.
    pub fn set_text<S: Into<String>>(&mut self, value: S) -> &mut Element {
        let value = value.into();
        if value.is_empty() {
            self.text = None;
        } else {
            self.text = Some(value);
        }
        self
    }

    pub fn write_end_tag(mut self, value: bool) -> Self {
        self.write_end_tag = value;

        self
    }

    /// Returns the tail text of a tag.
    ///
    /// The tail is the text following an element.
    pub fn tail(&self) -> &str {
        self.tail.as_ref().map(|x| x.as_str()).unwrap_or("")
    }

    /// Sets a new tail text value for the tag.
    pub fn set_tail<S: Into<String>>(&mut self, value: S) -> &mut Element {
        let value = value.into();
        if value.is_empty() {
            self.tail = None;
        } else {
            self.tail = Some(value);
        }
        self
    }

    /// The tag of the element as qualified name.
    ///
    /// Use the `QName` functionality to extract the information from the
    /// tag name you care about (like the local name).
    pub fn tag(&self) -> &QName {
        &self.tag
    }

    /// Sets a new tag for the element.
    pub fn set_tag<'a>(&mut self, tag: &QName<'a>) -> &mut Element {
        self.tag = tag.share();
        self
    }

    /// Returns the number of children
    pub fn child_count(&self) -> usize {
        self.children.len()
    }

    /// Returns the nth child.
    pub fn get_child(&self, idx: usize) -> Option<&Element> {
        self.children.get(idx)
    }

    /// Returns the nth child as a mutable reference.
    pub fn get_child_mut(&mut self, idx: usize) -> Option<&mut Element> {
        self.children.get_mut(idx)
    }

    /// Removes a child.
    ///
    /// This returns the element if it was removed or None if the
    /// index was out of bounds.
    pub fn remove_child(&mut self, idx: usize) -> Option<Element> {
        if self.children.len() > idx {
            Some(self.children.remove(idx))
        } else {
            None
        }
    }

    /// Appends a new child and returns a reference to self.
    pub fn append_child(&mut self, child: Element) -> &mut Element {
        self.children.push(child);
        self
    }

    /// Appends a new child to the element and returns a reference to it.
    ///
    /// This uses ``Element::new_with_namespaces`` internally and can
    /// then be used like this:
    ///
    /// ```
    /// use xmpp_xml::Element;
    ///
    /// let ns = "http://example.invalid/#ns";
    /// let mut root = Element::new((ns, "mydoc"));
    ///
    /// {
    ///     let mut list = root.append_new_child((ns, "list"));
    ///     for x in 0..3 {
    ///         list.append_new_child((ns, "item")).set_text(format!("Item {}", x));
    ///     }
    /// }
    /// ```
    pub fn append_new_child<'a, Q: AsQName<'a>>(&'a mut self, tag: Q) -> &'a mut Element {
        let child = Element::new_with_namespaces(tag, self);
        self.append_child(child);
        let idx = self.children.len() - 1;
        &mut self.children[idx]
    }

    /// Returns an iterator over all children.
    pub fn children<'a>(&'a self) -> Children<'a> {
        Children { idx: 0, element: self }
    }

    /// Returns a mutable iterator over all children.
    pub fn children_mut<'a>(&'a mut self) -> ChildrenMut<'a> {
        ChildrenMut { iter: self.children.iter_mut() }
    }

    /// Returns all children with the given name.
    pub fn find_all<'a, Q: AsQName<'a>>(&'a self, tag: Q) -> FindChildren<'a> {
        FindChildren {
            tag: tag.as_qname(),
            child_iter: self.children(),
        }
    }

    /// Returns all children with the given name.
    pub fn find_all_mut<'a, Q: AsQName<'a>>(&'a mut self, tag: Q) -> FindChildrenMut<'a> {
        FindChildrenMut {
            tag: tag.as_qname(),
            child_iter: self.children_mut(),
        }
    }

    /// Finds the first matching child
    pub fn find<'a, Q: AsQName<'a>>(&'a self, tag: Q) -> Option<&'a Element> {
        use std::borrow::Borrow;
        let tag = tag.as_qname();

        for child in self.children() {
            if child.tag() == tag.borrow() {
                return Some(child);
            }
        }
        None
    }

    /// Finds the first matching child and returns a mut ref
    pub fn find_mut<'a, Q: AsQName<'a>>(&'a mut self, tag: Q) -> Option<&'a mut Element> {
        self.find_all_mut(tag).next()
    }

    /// Look up an attribute by qualified name.
    pub fn get_attr<'a, Q: AsQName<'a>>(&'a self, name: Q) -> Option<&'a str> {
        self.attributes.get(&name.as_qname()).map(|x| x.as_str())
    }

    /// Sets a new attribute.
    ///
    /// This returns a reference to the element so you can chain the calls.
    pub fn set_attr<'a, Q: AsQName<'a>, S: Into<String>>(&'a mut self, name: Q, value: S) -> &'a mut Element {
        self.attributes.insert(name.as_qname().share(), value.into());
        self
    }

    /// Removes an attribute and returns the stored string.
    pub fn remove_attr<'a, Q: AsQName<'a>>(&'a mut self, name: Q) -> Option<String> {
        // so this requires some explanation.  We store internally QName<'static>
        // which means the QName has a global lifetime.  This works because we
        // move the internal string storage into a global string cache or we are
        // pointing to static memory in the binary.
        //
        // However while Rust can coerce our BTreeMap from QName<'static> to
        // QName<'a> when reading, we can't do the same when writing.  This is
        // to prevent us from stashing a QName<'a> into the btreemap.  However on
        // remove that restriction makes no sense so we can unsafely transmute it
        // away.  I wish there was a better way though.
        use std::borrow::Borrow;
        let name = name.as_qname();
        let name_ref: &QName<'a> = name.borrow();
        let name_ref_static: &QName<'static> = unsafe { mem::transmute(name_ref) };
        self.attributes.remove(name_ref_static)
    }

    /// Returns an iterator over all attributes
    pub fn attrs<'a>(&'a self) -> Attrs<'a> {
        Attrs { iter: self.attributes.iter() }
    }

    /// Count the attributes
    pub fn attr_count(&self) -> usize {
        self.attributes.len()
    }

    fn get_nsmap_mut(&mut self) -> &mut NamespaceMap {
        let new_map = match self.nsmap {
            Some(ref mut nsmap) if Rc::strong_count(nsmap) == 1 => None,
            Some(ref mut nsmap) => Some(Rc::new((**nsmap).clone())),
            None => Some(Rc::new(NamespaceMap::new())),
        };
        if let Some(nsmap) = new_map {
            self.nsmap = Some(nsmap);
        }
        Rc::get_mut(self.nsmap.as_mut().unwrap()).unwrap()
    }

    /// Registers a namespace with the internal namespace map.
    ///
    /// Note that there is no API to remove namespaces from an element once
    /// the namespace has been set so be careful with modifying this!
    ///
    /// This optionally also registers a specific prefix however if that prefix
    /// is already used a random one is used instead.
    pub fn register_namespace(&mut self, url: &str, prefix: Option<&str>) {
        if self.get_namespace_prefix(url).is_none() {
            if self.get_nsmap_mut().register_if_missing(url, prefix) {
                self.emit_nsmap = true;
            }
        }
    }

    /// Sets a specific namespace prefix.  This will also register the
    /// namespace if it was unknown so far.
    ///
    /// In case a prefix is set that is already set elsewhere an error is
    /// returned.  It's recommended that this method is only used on the
    /// root node before other prefixes are added.
    pub fn set_namespace_prefix(&mut self, url: &str, prefix: &str) -> Result<(), Error> {
        if self.get_namespace_prefix(url) == Some(prefix) {
            Ok(())
        } else {
            self.get_nsmap_mut().set_prefix(url, prefix)
        }
    }

    /// Returns the assigned prefix for a namespace.
    pub fn get_namespace_prefix(&self, url: &str) -> Option<&str> {
        match url {
            NS_EMPTY_URI => Some(""),
            NS_XML_URI => Some("xml"),
            NS_XMLNS_URI => Some("xmlns"),
            _ => {
                if let Some(ref nsmap) = self.nsmap {
                    nsmap.get_prefix(url)
                } else {
                    None
                }
            }
        }
    }

    /// Finds the first element that match a given path downwards
    pub fn navigate<'a, Q: AsQName<'a>>(&'a self, path: &[Q]) -> Option<&'a Element> {
        use std::borrow::Borrow;
        let mut node = self;

        'outer: for piece in path {
            let reftag = piece.as_qname();
            for child in node.children() {
                if child.tag() == reftag.borrow() {
                    node = child;
                    continue 'outer;
                }
            }
            return None;
        }

        Some(node)
    }
}
