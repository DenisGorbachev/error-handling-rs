use thiserror::Error;

/// Associates an error with the item that caused it.
#[derive(Error, Debug)]
#[error("error occurred for item {item}: {source}")]
pub struct ItemError<T, E> {
    /// The item that produced the error.
    pub item: T,
    /// The error produced for the item.
    pub source: E,
}
