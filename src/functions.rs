mod get_root_error;

pub use get_root_error::*;

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
