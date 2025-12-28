use core::fmt::{Debug, Display, Formatter};

/// A wrapper that renders `Display` using the inner type's `Debug` implementation.
#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug)]
pub struct DisplayAsDebug<T: Debug>(
    /// Inner value rendered with `Debug` for `Display`.
    pub T,
);

impl<T: Debug> Display for DisplayAsDebug<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl<T: Debug> From<T> for DisplayAsDebug<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}
