use crate::{Display, Error};

#[derive(Error, Display, Debug)]
pub struct ItemError<T, E> {
    pub item: T,
    #[error(source)]
    pub source: E,
}
