use core::fmt::Formatter;

pub trait Errgonomic {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result;
}
