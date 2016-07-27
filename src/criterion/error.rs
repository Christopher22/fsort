use std::fmt::{Display, Formatter, Result as FmtResult};
use std::error::Error as ErrorTrait;
use std::path::Path;
use std::fs::Metadata;

#[derive(Debug,Clone,Copy)]
/// An error occurring during the creation of a criterion.
pub enum Error {
    /// It was not possible to access file behind the path.
    NoAccess,
    /// The path is not a file but a directory, device or other unsupported handle.
    InvalidPath,
    /// The requested search criterion is not supported on this OS.
    CriterionUnsupported
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.description())
    }
}

impl ErrorTrait for Error {
    fn description(&self) -> &str {
        match *self {
            Error::NoAccess => "Missing access privileges",
            Error::InvalidPath => "Path is not a file",
            Error::CriterionUnsupported => "Sort criterion unsupported by this OS"
        }
    }
}

/// Reads metadata from a file.
pub fn try_metadata<P : AsRef<Path>>(path : P) -> Result<Metadata, Error> {
    if let Ok(meta) = path.as_ref().metadata() {
        if !meta.is_file() {
            Err(Error::InvalidPath)
        } else {
            Ok(meta)
        }
    }
        else {
        Err(Error::NoAccess)
    }
}

