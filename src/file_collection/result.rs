use ::criterion::Error;

/// The enum describes the result of a file adding operation.
#[derive(Debug)]
pub enum Result {
    /// The given path is not a valid directory.
    InvalidStartingPoint,
    /// The operation was successful and the files was added.
    Success(u32),
    /// While some files was added correctly there occurs several error during operation.
    FileError(u32, Vec<Error>)
}