//! This module contains different criteria for sorting files.

use std::cmp::Ord;
use std::path::Path;
use std::fmt::Debug;

mod error;
mod access;
mod creation;
mod modify;
mod name;
mod size;

// Imports structs to the module
pub use self::error::Error;
pub use self::access::AccessDate;
pub use self::creation::CreationDate;
pub use self::modify::ModifyDate;
pub use self::name::FileName;
pub use self::size::FileSize;

/// A trait for objects which are valid sorting criteria for files.
pub trait Criterion: Sized + Ord + Debug {

    /// Reads a specific criterion from a file.
    fn from_path<P : AsRef<Path>>(path : P) -> Result<Self, Error>;
}