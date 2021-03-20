use std::{borrow::Cow, fmt, io, str::Utf8Error};

use xml::reader::{Error as XmlReadError, ErrorKind as XmlReadErrorKind};
use xml::writer::Error as XmlWriteError;

use crate::Position;

/// Errors that can occur parsing XML
#[derive(Debug)]
pub enum Error {
    /// The XML is invalid
    MalformedXml { msg: Cow<'static, str>, pos: Position },
    /// An IO Error
    Io(io::Error),
    /// A UTF-8 Error
    Utf8(Utf8Error),
    /// This library is unable to process this XML. This can occur if, for
    /// example, the XML contains processing instructions.
    UnexpectedEvent { msg: Cow<'static, str>, pos: Position },
    /// A namespace prefix was already used
    DuplicateNamespacePrefix,
}

impl Error {
    /// Returns the position of the error if known
    pub fn position(&self) -> Option<Position> {
        match self {
            &Error::MalformedXml { pos, .. } => Some(pos),
            &Error::UnexpectedEvent { pos, .. } => Some(pos),
            _ => None,
        }
    }

    /// Returns the line number of the error or 0 if unknown
    pub fn line(&self) -> u64 {
        self.position().map(|x| x.line()).unwrap_or(0)
    }

    /// Returns the column of the error or 0 if unknown
    pub fn column(&self) -> u64 {
        self.position().map(|x| x.column()).unwrap_or(0)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Error::MalformedXml { ref pos, ref msg } => write!(f, "Malformed XML: {} ({})", msg, pos),
            &Error::Io(ref e) => write!(f, "{}", e),
            &Error::Utf8(ref e) => write!(f, "{}", e),
            &Error::UnexpectedEvent { ref msg, .. } => write!(f, "Unexpected XML event: {}", msg),
            &Error::DuplicateNamespacePrefix => write!(f, "Encountered duplicated namespace prefix"),
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match self {
            &Error::MalformedXml { .. } => "Malformed XML",
            &Error::Io(..) => "IO error",
            &Error::Utf8(..) => "utf-8 error",
            &Error::UnexpectedEvent { .. } => "Unexpected XML element",
            &Error::DuplicateNamespacePrefix => "Duplicated namespace prefix",
        }
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        match self {
            &Error::Io(ref e) => Some(e),
            &Error::Utf8(ref e) => Some(e),
            _ => None,
        }
    }
}

impl From<XmlReadError> for Error {
    fn from(err: XmlReadError) -> Error {
        match err.kind() {
            &XmlReadErrorKind::Io(ref err) => Error::Io(io::Error::new(err.kind(), err.to_string())),
            &XmlReadErrorKind::Utf8(ref err) => Error::Utf8(err.clone()),
            &XmlReadErrorKind::UnexpectedEof => Error::Io(io::Error::new(io::ErrorKind::UnexpectedEof, "Encountered unexpected eof")),
            &XmlReadErrorKind::Syntax(ref msg) => Error::MalformedXml {
                msg: msg.clone(),
                pos: Position::from_xml_position(&err),
            },
        }
    }
}

impl From<XmlWriteError> for Error {
    fn from(err: XmlWriteError) -> Error {
        match err {
            XmlWriteError::Io(err) => Error::Io(err),
            err => {
                return Err(err).unwrap();
            }
        }
    }
}

impl From<Error> for std::io::Error {
    fn from(e: Error) -> Self {
        std::io::Error::new(std::io::ErrorKind::Other, e.to_string())
    }
}
