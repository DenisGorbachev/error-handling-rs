use crate::DisplayAsDebug;
use std::path::PathBuf;

/// A [`PathBuf`] that returns a `Debug` representation in [`Display`](std::fmt::Display) impl.
pub type PathBufDisplay = DisplayAsDebug<PathBuf>;
