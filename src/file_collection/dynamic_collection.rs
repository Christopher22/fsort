use std::path::{Path, PathBuf};

use ::criterion::*;
use ::Order;

use super::{Result, ErrorBehavior, PathIter, FileCollection, StaticCollection};

/// A sorted collection of files with changeable criterion.
pub struct DynamicCollection {
    collection : Box<FileCollection<IntoIter=PathIter,Item=PathBuf> + 'static>
}

impl DynamicCollection {

    /// Creates a new sorted collection with a specific criterion.
    /// # Examples
    /// ```
    /// use fsort::criterion::FileName;
    /// use fsort::file_collection::DynamicCollection;
    ///
    /// let collection = DynamicCollection::new::<FileName>();
    /// ```
    pub fn new<T : Criterion + 'static>() -> DynamicCollection {
        DynamicCollection {
            collection : Box::new(StaticCollection::<T>::new())
        }
    }

    /// Changes the sorting criterion.
    /// # Examples
    /// ```
    /// use std::fs::File;
    /// use std::path::PathBuf;
    /// use fsort::criterion::{FileName, FileSize};
    /// use fsort::file_collection::{FileCollection, DynamicCollection};
    ///
    /// // Creates temporal files
    /// let mut s1 = std::env::temp_dir();
    /// let mut s2 = std::env::temp_dir();
    /// s1.push("S1.tmp");
    /// s2.push("S2.tmp");
    /// File::create(&s1).unwrap().set_len(10);
    /// File::create(&s2).unwrap().set_len(5);
    ///
    /// // Inserts files into collection
    /// let mut collection = DynamicCollection::new::<FileName>();
    /// collection.add_file(&s2);
    /// collection.add_file(&s1);
    ///
    /// let mut iter_name = collection.path_iter();
    /// assert_eq!(s1, iter_name.next().unwrap());
    /// assert_eq!(s2, iter_name.next().unwrap());
    /// assert_eq!(None, iter_name.next());
    ///
    /// collection.set_criterion::<FileSize>();
    /// let mut iter_size = collection.path_iter();
    /// assert_eq!(s2, iter_size.next().unwrap());
    /// assert_eq!(s1, iter_size.next().unwrap());
    /// assert_eq!(None, iter_size.next());
    ///
    /// ```
    pub fn set_criterion<T : Criterion + 'static>(&mut self) {

        // Create new collection ...
        let mut new_collection = DynamicCollection::new::<T>();
        new_collection.set_order(self.collection.get_order());

        // ... and fill it with the paths of the current files.
        for file in self.path_iter() {
            new_collection.add_file(&file);
        }

        self.collection = Box::new(new_collection);
    }
}

impl FileCollection for DynamicCollection {
    fn add_file(&mut self, path: &Path) -> Result {
        self.collection.as_mut().add_file(path)
    }

    fn add_directory(&mut self, path: &Path, recursive : bool, error_behavior: ErrorBehavior) -> Result {
        self.collection.as_mut().add_directory(path, recursive, error_behavior)
    }

    fn set_order(&mut self, order: Order) {
        self.collection.set_order(order);
    }

    fn get_order(&self) -> Order {
        self.collection.get_order()
    }

    fn path_iter(&self) -> PathIter {
        self.collection.path_iter()
    }
}

impl IntoIterator for DynamicCollection {
    type Item = PathBuf;
    type IntoIter = PathIter;

    fn into_iter(self) -> PathIter {
        self.collection.path_iter()
    }
}
