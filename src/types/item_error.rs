use thiserror::Error;

#[derive(Error, Debug)]
#[error("error occurred for item {item}: {source}")]
pub struct ItemError<T, E> {
    pub item: T,
    pub source: E,
}
