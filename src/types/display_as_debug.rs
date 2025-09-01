use std::fmt::{Debug, Display, Formatter};

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug)]
pub struct DisplayAsDebug<T: Debug>(pub T);

impl<T: Debug> Display for DisplayAsDebug<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl<T: Debug> From<T> for DisplayAsDebug<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}
