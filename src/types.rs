mod debug_as_display;
mod display_as_debug;
mod display_debug_pair;
mod item_error;

pub use debug_as_display::*;
pub use display_as_debug::*;
pub use display_debug_pair::*;
pub use item_error::*;

use std::path::PathBuf;

pub type PathBufDisplay = DisplayAsDebug<PathBuf>;
