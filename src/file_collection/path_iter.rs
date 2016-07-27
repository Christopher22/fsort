use std::path::PathBuf;
use std::vec::IntoIter;

use ::criterion::Criterion;

use super::StaticCollection;

/// An iterator over the sorted paths of a FileCollection.
pub struct PathIter {
    iter : IntoIter<PathBuf>
}

impl PathIter {

    /// Creates a new iterator from a StaticCollection
    pub fn new<C : Criterion>(collection : &StaticCollection<C>) -> PathIter {

        // Copy the paths of the files into a vector ...
        let vector : Vec<PathBuf> = collection.iter().map(|x| x.path.clone()).collect();

        // ... and store its IntoIterator in the PathIter.
        PathIter {
            iter : vector.into_iter()
        }
    }
}

impl Iterator for PathIter {
    type Item = PathBuf;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl DoubleEndedIterator for PathIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }
}

impl ExactSizeIterator for PathIter {
    fn len(&self) -> usize {
        self.iter.len()
    }
}