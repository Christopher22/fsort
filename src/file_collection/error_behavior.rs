/// This enum describes the behavior of a sorting algorithm in case of errors while adding files.
#[derive(Debug,Clone,Copy)]
pub enum ErrorBehavior {
    /// Ignore the error and continue adding.
    Ignore,
    /// Abort the execution and stop adding files.
    Abort
}