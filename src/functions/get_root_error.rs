use std::error::Error;

/// Returns the deepest source error in the error chain (the root cause).
pub fn get_root_source(error: &dyn Error) -> &dyn Error {
    let mut source = error;
    while let Some(source_new) = source.source() {
        source = source_new;
    }
    source
}
