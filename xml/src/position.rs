use std::fmt;

use xml::common::Position as XmlPosition;

/// Represents a position in the source.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct Position {
    line: u64,
    column: u64,
}

impl Position {
    /// Creates a new position.
    pub fn new(line: u64, column: u64) -> Position {
        Position { line, column }
    }

    pub(crate) fn from_xml_position(pos: &dyn XmlPosition) -> Position {
        let pos = pos.position();
        Position::new(pos.row, pos.column)
    }

    /// Returns the line number of the position
    pub fn line(&self) -> u64 {
        self.line
    }
    /// Returns the column of the position
    pub fn column(&self) -> u64 {
        self.column
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}
