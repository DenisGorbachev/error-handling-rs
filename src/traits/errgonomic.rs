use core::fmt::Formatter;
use std::io;
use std::io::Write;

pub trait Errgonomic {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result;
}

pub trait DoWrite {
    fn do_write(&self, writer: &mut dyn Write) -> Result<(), io::Error>;
}

/// This trait must be used instead of [`Display`](core::fmt::Display) because `Display` has a generic `impl<T> Display for &T`, which conflicts with an "auto-deref trick" `impl ... for &T` that we're using
pub trait DoDisplay {
    fn fmt(&self, _f: &mut Formatter<'_>) -> core::fmt::Result;
}
