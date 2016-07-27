use std::collections::btree_set::Iter as BSetIterator;
use std::iter::Rev;

use ::File;
use ::criterion::Criterion;

/// An iterator over the files in a `FileCollection` according to the requested order.
pub struct FileIter<'a, C : Criterion + 'a> {
    data : FileIteratorData<'a, C>
}

impl<'a, C: Criterion> Iterator for FileIter<'a, C> {
    type Item = &'a File<C>;

    fn next(&mut self) -> Option<&'a File<C>> {
        match self.data {
            FileIteratorData::Ascending(ref mut value) => value.next(),
            FileIteratorData::Descending(ref mut value) => value.next()
        }
    }
}

impl<'a, C : Criterion> DoubleEndedIterator for FileIter<'a, C> {
    fn next_back(&mut self) -> Option<&'a File<C>> {
        match self.data {
            FileIteratorData::Ascending(ref mut value) => value.next_back(),
            FileIteratorData::Descending(ref mut value) => value.next_back()
        }
    }
}

impl<'a, C : Criterion> ExactSizeIterator for FileIter<'a, C> {
    fn len(&self) -> usize {
        match self.data {
            FileIteratorData::Ascending(ref value) => value.len(),
            FileIteratorData::Descending(ref value) => value.len()
        }
    }
}

/// The (private) data of a FileIter.
pub enum FileIteratorData<'a, C : Criterion + 'a> {
    Ascending(BSetIterator<'a, File<C>>),
    Descending(Rev<BSetIterator<'a, File<C>>>)
}

impl<'a, C : Criterion> FileIteratorData<'a, C> {
    pub fn iter(self) -> FileIter<'a, C> {
        FileIter::<C> {
            data : self
        }
    }

    /*pub fn new_iterator(data : FileIteratorData<'a, C>) -> FileIter<C> {
        FileIter::<C> {
            data : data
        }
    }*/
}
