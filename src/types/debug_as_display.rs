use std::fmt::{Debug, Display, Formatter};

/// This wrapper is needed for types that have an easy-to-understand `Display` impl but hard-to-understand `Debug` impl
#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub struct DebugAsDisplay<T: Display>(pub T);

impl<T: Display> Debug for DebugAsDisplay<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl<T: Display> Display for DebugAsDisplay<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl<T: Display> From<T> for DebugAsDisplay<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}
