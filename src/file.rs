use std::cmp::{Ord, PartialOrd, Ordering};
use std::path::{Path, PathBuf};

use ::criterion::{Criterion,Error};

/// A file which could be sorted according to a specific criterion.
#[derive(Debug,Eq,PartialEq)]
pub struct File<C : Criterion> {
    /// The path to the file in the local filesystem.
    pub path : PathBuf,
    data : C
}

impl<C : Criterion> File<C> {

    /// Creates a new sortable file from a path.
    pub fn new<P : AsRef<Path>>(path : P) -> Result<File<C>, Error> {
        let mut pathbuf = PathBuf::new();
        pathbuf.push(path);

        // Creates criterion and map it into the file.
        C::from_path(&pathbuf).map(move |x| {
            File::<C> {
                path : pathbuf,
                data : x
            }
        })
    }
}

impl<C : Criterion> Ord for File<C> {
    fn cmp(&self, other: &File<C>) -> Ordering {
        self.data.cmp(&other.data)
    }
}

impl<C : Criterion> PartialOrd for File<C> {
    fn partial_cmp(&self, other: &File<C>) -> Option<Ordering> {
        self.data.partial_cmp(&other.data)
    }
}

impl<C : Criterion> From<File<C>> for PathBuf {
    fn from(file : File<C>) -> PathBuf {
        file.path.to_path_buf()
    }
}