mod debug_as_display;
mod display_as_debug;

pub use debug_as_display::*;
pub use display_as_debug::*;
use std::path::PathBuf;

pub type PathBufDisplay = DisplayAsDebug<PathBuf>;
