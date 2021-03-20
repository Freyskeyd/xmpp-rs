/// Xml Prolog version handle by xmpp_xml
pub enum XmlProlog {
    Version10,
    Version11,
}

/// A struct that define write options.
pub struct WriteOptions {
    pub(crate) xml_prolog: Option<XmlProlog>,
    pub(crate) write_end_tag: bool,
}

impl Default for WriteOptions {
    fn default() -> WriteOptions {
        WriteOptions {
            xml_prolog: Some(XmlProlog::Version10),
            write_end_tag: true,
        }
    }
}

impl WriteOptions {
    pub fn new() -> WriteOptions {
        WriteOptions { ..WriteOptions::default() }
    }

    /// Define which xml prolog will be displayed when rendering an Element.
    ///
    /// Note that prolog is optional, an XML document with a missing prolog is well-formed but not valid.
    ///
    /// See RFC: [W3C XML 26 November 2008](https://www.w3.org/TR/xml/#sec-prolog-dtd)
    pub fn set_xml_prolog(mut self, prolog: Option<XmlProlog>) -> Self {
        self.xml_prolog = prolog;

        self
    }

    /// Define if we write the end tag of an element.
    pub fn set_write_end_tag(mut self, value: bool) -> Self {
        self.write_end_tag = value;

        self
    }
}
