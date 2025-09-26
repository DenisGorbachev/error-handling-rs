use crate::functions::write_to_named_temp_file;
use std::error::Error;

pub fn eprintln_error(error: &dyn Error) {
    eprintln!("- {}", error);
    let mut source = error;
    while let Some(source_new) = source.source() {
        eprintln!("- {}", source_new);
        source = source_new;
    }
    eprintln!();
    let error_debug = format!("{:#?}", error);
    let result = write_to_named_temp_file::write_to_named_temp_file(error_debug.as_bytes());
    match result {
        Ok((_file, path_buf)) => {
            eprintln!("Full error written to {}", path_buf.display());
        }
        Err(other_error) => {
            eprintln!("{other_error:#?}");
        }
    }
}
