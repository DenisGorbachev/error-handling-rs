use crate::ErrorDisplayer;
use core::error::Error;
use core::fmt::{Debug, Write};
use core::fmt::{Display, Formatter};
use core::ops::{Deref, DerefMut};

/// An owned collection of errors that itself implements [`Error`].
/// T must implement Display or Errgonomic.
#[derive(Default, Debug)]
pub struct ErrVec<E> {
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

impl<E> Deref for ErrVec<E> {
    type Target = Vec<E>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<E> DerefMut for ErrVec<E> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<E> ErrVec<E> {
    /// Builds an [`ErrVec`] by boxing each error from the iterator.
    pub fn new(iter: impl IntoIterator<Item = E>) -> Self {
        Self {
            inner: iter.into_iter().collect(),
        }
    }
}

impl<E> From<Vec<E>> for ErrVec<E> {
    fn from(inner: Vec<E>) -> Self {
        Self {
            inner,
        }
    }
}

impl<E: Clone, const N: usize> From<[E; N]> for ErrVec<E> {
    fn from(inner: [E; N]) -> Self {
        Self {
            inner: inner.to_vec(),
        }
    }
}

impl<E: Clone> From<&[E]> for ErrVec<E> {
    fn from(inner: &[E]) -> Self {
        Self {
            inner: inner.to_vec(),
        }
    }
}
