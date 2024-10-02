use std::fmt;
use std::io;
use syn::Error as SynError;
use ra_ap_ide_assists::Assist;
use ra_ap_vfs::AbsPathBuf;

#[derive(Debug)]
pub enum ExtractionError {
    Io(io::Error),
    Parse(SynError),
    InvalidManifest,
    InvalidStartIdx,
    InvalidEndIdx,
    SameIdx,
    InvalidIdxPair,
    NoAvailableAssist,
    NoExtractFunction(Vec<Assist>),
    ControlFlowFixupFailed(AbsPathBuf),
}

impl fmt::Display for ExtractionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExtractionError::Io(e) => write!(f, "I/O error: {}", e),
            ExtractionError::Parse(e) => write!(f, "Parse error: {}", e),
            ExtractionError::InvalidManifest => write!(f, "Could not find a manifest file for the given path"),
            ExtractionError::InvalidStartIdx => write!(f, "Invalid start index"),
            ExtractionError::InvalidEndIdx => write!(f, "Invalid end index"),
            ExtractionError::SameIdx => write!(f, "Start and end indices are the same"),
            ExtractionError::InvalidIdxPair => write!(f, "Invalid pair of start and end indices"),
            ExtractionError::NoAvailableAssist => write!(f, "No Available Assists found for the given selection"),
            ExtractionError::NoExtractFunction(assists) => write!(f, "No Extract Function Assist found for the given selection of assists {:?}", assists),
            ExtractionError::ControlFlowFixupFailed(path) => write!(f, "Control Flow Fixup failed for file {:?}", path),
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