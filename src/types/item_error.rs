use derive_more::Error;
use fmt_derive::Display;

#[derive(Error, Display, Debug)]
pub struct ItemError<T, E> {
    pub item: T,
    #[error(source)]
    pub source: E,
}
