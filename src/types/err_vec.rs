use std::error::Error;
use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};

#[derive(Default, Debug)]
pub struct ErrVec {
    pub inner: Vec<Box<dyn Error + 'static>>,
}

impl Display for ErrVec {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("encountered {len} errors", len = self.inner.len()))
    }
}

impl Error for ErrVec {}

impl Deref for ErrVec {
    type Target = Vec<Box<dyn Error + 'static>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for ErrVec {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl ErrVec {
    pub fn new<E: Error + 'static>(iter: impl IntoIterator<Item = E>) -> Self {
        Self {
            inner: iter
                .into_iter()
                .map(|err| Box::new(err) as Box<dyn Error + 'static>)
                .collect(),
        }
    }
}

impl<E: Error + 'static> From<Vec<E>> for ErrVec {
    fn from(value: Vec<E>) -> Self {
        Self::new(value)
    }
}
