use std::time::SystemTime;
use std::path::Path;
use std::convert::From;

use super::{Error, Criterion};
use super::error::try_metadata;

/// A criterion which sorts files by their last dates of modify.
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct ModifyDate {
    modify_date : SystemTime
}

impl Criterion for ModifyDate {

    fn from_path<P : AsRef<Path>>(path : P) -> Result<ModifyDate, Error> {

        // Gets metadata if path is a valid file
        try_metadata(path).and_then(|meta| {
            // Creates criterion if date of last modify is accessible, ...
            meta.modified().map(|modified| {
                ModifyDate {
                    modify_date: modified
                }
            // ... or returns an error elsewhere.
            }).map_err(|_| Error::CriterionUnsupported)
        })
    }
}

impl From<ModifyDate> for SystemTime {
    fn from(modify: ModifyDate) -> Self {
        modify.modify_date
    }
}