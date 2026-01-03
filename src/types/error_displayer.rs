use crate::writeln_error_to_formatter;
use core::fmt::{Display, Formatter};
use std::error::Error;

pub struct ErrorDisplayer<'a, E: ?Sized>(pub &'a E);

impl<'a, E: Error + ?Sized> Display for ErrorDisplayer<'a, E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        writeln_error_to_formatter(self.0, f)
    }
}

impl<'a, E: Error + ?Sized> From<&'a E> for ErrorDisplayer<'a, E> {
    fn from(error: &'a E) -> Self {
        Self(error)
    }
}
