//! This modules contains the sorted `FileCollection`s and their iterators.

use std::path::{Path, PathBuf};

use ::Order;

mod dynamic_collection;
mod static_collection;
mod path_iter;
mod file_iter;
mod error_behavior;
mod result;

pub use self::static_collection::StaticCollection;
pub use self::dynamic_collection::DynamicCollection;
pub use self::path_iter::PathIter;
pub use self::file_iter::FileIter;
pub use self::error_behavior::ErrorBehavior;
pub use self::result::Result;

/// A trait for objects which are sorted collections of files.
pub trait FileCollection : IntoIterator<Item=PathBuf,IntoIter=PathIter> {

    /// Adds a file to the collection.
    /// # Examples
    /// See `DynamicCollection::set_criterion`
    fn add_file(&mut self, path: &Path) -> Result;

    /// Adds all files of a directory to the collection.
    fn add_directory(&mut self, path: &Path, recursive : bool, error_behavior: ErrorBehavior) -> Result;

    /// Returns the current order of the collection.
    fn get_order(&self) -> Order;

    /// Sets the current order of the collection.
    fn set_order(&mut self, order : Order);

    /// Returns an iterator about all the (sorted) paths in the collection according to the order.
    /// # Examples
    /// See `DynamicCollection::set_criterion`
    fn path_iter(&self) -> PathIter;
}