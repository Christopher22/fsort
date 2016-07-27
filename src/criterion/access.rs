use std::time::SystemTime;
use std::path::Path;
use std::convert::From;

use super::{Error, Criterion};
use super::error::try_metadata;

/// A criterion which sorts files by the dates of the last access to them.
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct AccessDate {
    access_date: SystemTime
}

impl Criterion for AccessDate {

    fn from_path<P: AsRef<Path>>(path: P) -> Result<AccessDate, Error> {

        // Gets metadata if path is a valid file
        try_metadata(path).and_then(|meta| {
            // Create criterion if access date is accessible, ...
            meta.accessed().map(|access| {
                AccessDate {
                    access_date: access
                }
            // ... or returns an error elsewhere.
            }).map_err(|_| Error::CriterionUnsupported)
        })
    }
}

impl From<AccessDate> for SystemTime {
    fn from(access: AccessDate) -> Self {
        access.access_date
    }
}