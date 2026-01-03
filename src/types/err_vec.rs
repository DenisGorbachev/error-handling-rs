use crate::ErrorDisplayer;
use core::error::Error;
use core::fmt::{Debug, Write};
use core::fmt::{Display, Formatter};
use core::ops::{Deref, DerefMut};

/// An owned collection of errors
#[derive(Default, Clone, Debug)]
pub struct ErrVec<E: Error> {
    /// Collected errors.
    pub inner: Vec<E>,
}

impl<E: Error> Display for ErrVec<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "encountered {len} errors", len = self.len())?;
        self.inner.iter().try_for_each(|error| {
            f.write_char('\n')?;
            let recursive_displayer = ErrorDisplayer(error);
            let string = format!("{recursive_displayer}");
            let mut lines = string.lines();
            let first_line_opt = lines.next();
            if let Some(first_line) = first_line_opt {
                write!(f, "  * {first_line}")?;
                lines.try_for_each(|line| write!(f, "\n    {line}"))?;
            }
            Ok(())
        })
    }
}

impl<E: Error> Error for ErrVec<E> {}

impl<E: Error> Deref for ErrVec<E> {
    type Target = Vec<E>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<E: Error> DerefMut for ErrVec<E> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<E: Error> ErrVec<E> {
    /// Builds an [`ErrVec`] by boxing each error from the iterator.
    pub fn new(iter: impl IntoIterator<Item = E>) -> Self {
        Self {
            inner: iter.into_iter().collect(),
        }
    }
}

impl<E: Error> From<Vec<E>> for ErrVec<E> {
    fn from(inner: Vec<E>) -> Self {
        Self {
            inner,
        }
    }
}

impl<E: Error + Clone, const N: usize> From<[E; N]> for ErrVec<E> {
    fn from(inner: [E; N]) -> Self {
        Self {
            inner: inner.to_vec(),
        }
    }
}

impl<E: Error + Clone> From<&[E]> for ErrVec<E> {
    fn from(inner: &[E]) -> Self {
        Self {
            inner: inner.to_vec(),
        }
    }
}
