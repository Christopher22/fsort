//! fsort is a crate to sort files in a fast, OS-independent and "rusty" way.

mod file;
mod order;

pub mod file_collection;
pub mod criterion;
pub use self::file::File;
pub use self::order::Order;