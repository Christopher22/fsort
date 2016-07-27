use std::time::SystemTime;
use std::path::Path;
use std::convert::From;

use super::{Error, Criterion};
use super::error::try_metadata;

/// A criterion which sorts files by their dates of creation.
/// # Examples
///
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct CreationDate {
    creation_date : SystemTime
}

impl Criterion for CreationDate {

    fn from_path<P : AsRef<Path>>(path : P) -> Result<CreationDate, Error> {

        // Gets metadata if path is a valid file
        try_metadata(path).and_then(|meta| {
            // Creates criterion if creation date is accessible, ...
            meta.created().map(|creation| {
                CreationDate {
                    creation_date: creation
                }
            // ... or returns an error elsewhere.
            }).map_err(|_| Error::CriterionUnsupported)
        })
    }
}

impl From<CreationDate> for SystemTime {
    fn from(creation: CreationDate) -> Self {
        creation.creation_date
    }
}