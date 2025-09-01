mod display_as_debug;

pub use display_as_debug::*;
use std::path::PathBuf;

pub type PathBufDisplay = DisplayAsDebug<PathBuf>;
