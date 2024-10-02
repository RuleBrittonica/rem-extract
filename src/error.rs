use std::fmt;
use std::io;
use syn::Error as SynError;

#[derive(Debug)]
pub enum ExtractionError {
    Io(io::Error),
    Parse(SynError),
    InvalidManifest,
    InvalidLineRange,
    InvalidColumnRange,
    ZeroLineIndex,
    SameCursor,
}

impl fmt::Display for ExtractionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExtractionError::Io(e) => write!(f, "I/O error: {}", e),
            ExtractionError::Parse(e) => write!(f, "Parse error: {}", e),
            ExtractionError::InvalidManifest => write!(f, "Could not find a manifest file for the given path"),
            ExtractionError::InvalidLineRange => write!(f, "Invalid line range"),
            ExtractionError::InvalidColumnRange => write!(f, "Invalid column range"),
            ExtractionError::ZeroLineIndex => write!(f, "Line index must be greater than 0. Cursor is 1-indexed."),
            ExtractionError::SameCursor => write!(f, "Start and end cursors are the same"),
        }
    }
}

impl From<io::Error> for ExtractionError {
    fn from(error: io::Error) -> Self {
        ExtractionError::Io(error)
    }
}

impl From<SynError> for ExtractionError {
    fn from(error: SynError) -> Self {
        ExtractionError::Parse(error)
    }
}