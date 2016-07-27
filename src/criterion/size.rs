use std::path::Path;
use std::convert::From;

use super::{Error, Criterion};
use super::error::try_metadata;

/// A criterion which sorts files by their size.
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct FileSize {
    size : u64
}

impl Criterion for FileSize {
    fn from_path<P : AsRef<Path>>(path : P) -> Result<FileSize, Error> {

        // Gets metadata and creates criterion if possible
        try_metadata(path).map(|meta| {
            FileSize {
                size : meta.len()
            }
        })
    }
}

impl From<FileSize> for u64 {
    fn from(file_size: FileSize) -> Self {
        file_size.size
    }
}