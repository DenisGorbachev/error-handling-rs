use std::fmt::{Debug, Display};

#[derive(Clone, Debug)]
pub struct DisplayDebugPair<T: Display + Debug> {
    pub display: String,
    pub debug: T,
}

impl<T: Display + Debug> From<T> for DisplayDebugPair<T> {
    fn from(value: T) -> Self {
        Self {
            display: value.to_string(),
            debug: value,
        }
    }
}
