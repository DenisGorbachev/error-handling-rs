use crate::writeln_error_to_formatter;
use core::error::Error;
use core::fmt::Debug;
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
        writeln!(f, "encountered {len} errors", len = self.len())?;
        self.inner.iter().try_for_each(|error| {
            let recursive_displayer = RecursiveDisplayer {
                error,
                is_nested: true,
            };
            let string = format!("{recursive_displayer}");
            let mut lines = string.lines();
            let first_line_opt = lines.next();
            if let Some(first_line) = first_line_opt {
                writeln!(f, "  * {first_line}")?;
                lines.try_for_each(|line| writeln!(f, "    {line}"))?;
                writeln!(f)?;
            }
            Ok(())
        })
    }
}

pub struct RecursiveDisplayer<'a, E: ?Sized> {
    pub error: &'a E,
    pub is_nested: bool,
}

impl<'a, E: Error + ?Sized> Display for RecursiveDisplayer<'a, E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        writeln_error_to_formatter(self.error, self.is_nested, f)
    }
}

impl<'a, E: Error + ?Sized> From<&'a E> for RecursiveDisplayer<'a, E> {
    fn from(error: &'a E) -> Self {
        Self {
            error,
            is_nested: false,
        }
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
