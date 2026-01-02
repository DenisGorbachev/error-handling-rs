mod get_root_error;
mod partition_result;

pub use get_root_error::*;
pub use partition_result::*;

cfg_if::cfg_if! {
    if #[cfg(feature = "std")] {
        mod writeln_error;
        mod write_to_named_temp_file;
        mod exit_result;
        pub use writeln_error::*;
        pub use write_to_named_temp_file::*;
        pub use exit_result::*;
    }
}
mod fmt;
pub use fmt::*;
