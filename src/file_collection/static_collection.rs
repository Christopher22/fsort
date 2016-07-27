use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use ::criterion::{Error, Criterion};
use ::File;
use ::Order;

use super::Result;
use super::ErrorBehavior;
use super::FileIter;
use super::PathIter;
use super::file_iter::FileIteratorData;
use super::FileCollection;

/// A sorted collection of files with fixed criterion.
pub struct StaticCollection<C : Criterion> {
    order: Order,
    files: BTreeSet<File<C>>
}

impl<C : Criterion> StaticCollection<C> {

    /// Creates a new sorted collection with a specific criterion.
    /// # Examples
    /// ```
    /// use fsort::criterion::FileName;
    /// use fsort::file_collection::StaticCollection;
    ///
    /// let collection = StaticCollection::<FileName>::new();
    /// ```
    pub fn new() -> StaticCollection<C> {
        StaticCollection::<C> {
            order: Order::Ascending,
            files: BTreeSet::new()
        }
    }

    /// Returns an iterator over the sorted files in the collection according to the criterion.
    /// # Examples
    /// ```
    /// use std::fs::File;
    /// use std::path::PathBuf;
    /// use fsort::criterion::FileName;
    /// use fsort::file_collection::StaticCollection;
    /// use fsort::file_collection::FileCollection;
    ///
    /// let mut i1 = std::env::temp_dir();
    /// let mut i2 = std::env::temp_dir();
    /// i1.push("I1.tmp");
    /// i2.push("I2.tmp");
    /// File::create(&i1).unwrap();
    /// File::create(&i2).unwrap();
    ///
    /// // Inserts files into collection
    /// let mut collection = StaticCollection::<FileName>::new();
    /// collection.add_file(&i2);
    /// collection.add_file(&i1);
    ///
    /// let mut iter = collection.iter();
    /// assert_eq!(i1, iter.next().unwrap().path);
    /// assert_eq!(i2, iter.next().unwrap().path);
    /// assert_eq!(None, iter.next());
    /// ```
    pub fn iter(&self) -> FileIter<C> {
        (if let Order::Ascending = self.order {
            FileIteratorData::Ascending(self.files.iter())
        } else {
            FileIteratorData::Descending(self.files.iter().rev())
        }).iter()
    }
}

impl<C : Criterion> FileCollection for StaticCollection<C> {

    fn add_file(&mut self, path: &Path) -> Result {

        // Try to create a new file ...
        match File::<C>::new(path) {

            // ... and add it iff the file does not exist already in the collection.
            Ok(file) => Result::Success(if self.files.insert(file) { 1 } else { 0 }),
            Err(err) => Result::FileError(0, vec![err])
        }
    }

    fn add_directory(&mut self, path: &Path, recursive : bool, error_behavior: ErrorBehavior) -> Result {

        // Check if path is a valid directory
        if let Ok(dir) = path.read_dir() {

            let mut i: u32 = 0;
            let mut error: Vec<Error> = Vec::new();

            // For each valid element in dir ...
            for entity in dir.filter_map(move |x| x.ok()) {

                // Get type of entity
                let entity_type = if let Ok(entity_type_inner) = entity.file_type() {
                    entity_type_inner
                } else { continue; };

                // Add file(s) depending on type and return result
                let file_result = if entity_type.is_file() {
                        self.add_file(entity.path().as_path())
                    }
                    else if entity_type.is_dir() && recursive {
                        self.add_directory(entity.path().as_path(), recursive, error_behavior)
                    }
                    else { continue; };

                // Process result
                match file_result {

                    // Add successful edited files on success...
                    Result::Success(j) => { i += j; },

                    // ... or add the failed, too.
                    Result::FileError(j, mut inner_error) => {
                        i += j;
                        error.append(&mut inner_error);

                        // Abort adding if requested
                        if let ErrorBehavior::Abort = error_behavior {
                            return Result::FileError(i, error);
                        };
                    }
                    _ => { continue; }
                }
            }

            // Return the final result
            if error.len() == 0 { Result::Success(i) }
                else { Result::FileError(i, error) }
        }
        else {
            Result::InvalidStartingPoint
        }
    }

    fn set_order(&mut self, order: Order) {
        self.order = order;
    }

    fn get_order(&self) -> Order {
        self.order
    }

    fn path_iter(&self) -> PathIter {
        PathIter::new(self)
    }
}

impl<C : Criterion> IntoIterator for StaticCollection<C> {
    type Item = PathBuf;
    type IntoIter = PathIter;

    fn into_iter(self) -> PathIter {
        self.path_iter()
    }
}

impl<C : Criterion> Default for StaticCollection<C> {
    fn default() -> StaticCollection<C> {
        StaticCollection::<C> {
            order : Order::Ascending,
            files : BTreeSet::new()
        }
    }
}