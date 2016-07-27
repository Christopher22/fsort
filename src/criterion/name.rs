use std::ffi::OsString;
use std::path::Path;
use std::convert::From;

use super::{Error, Criterion};

/// A criterion which sorts files by their names.
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct FileName {
    name: OsString
}

impl Criterion for FileName {
    fn from_path<P: AsRef<Path>>(path: P) -> Result<FileName, Error> {
        // Reads file name
        match path.as_ref().file_name() {
            // Returns criterion if the path is valid and points to a file...
            Some(file_name) if path.as_ref().is_file() => Ok(FileName {
                name: file_name.to_os_string()
            }),
            // ... or an error if not.
            _ => Err(Error::InvalidPath)
        }
    }
}

impl From<FileName> for OsString {
    fn from(file_name: FileName) -> Self {
        file_name.name
    }
}