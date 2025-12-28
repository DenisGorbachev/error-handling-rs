use crate::{handle, map_err};
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use tempfile::{NamedTempFile, PersistError};
use thiserror::Error;

/// Writes the provided buffer to a named temporary file and persists it to disk.
///
/// Returns the persisted file handle and its path.
pub fn write_to_named_temp_file(buf: &[u8]) -> Result<(File, PathBuf), WriteErrorDebugToTempFileError> {
    use WriteErrorDebugToTempFileError::*;
    let mut temp = handle!(NamedTempFile::new(), CreateTempFileFailed);
    handle!(temp.write_all(buf), WriteFailed);
    map_err!(temp.keep(), KeepFailed)
}

/// Errors returned by [`write_to_named_temp_file`].
#[derive(Error, Debug)]
pub enum WriteErrorDebugToTempFileError {
    /// Failed to create a temporary file.
    #[error("failed to create a temporary file")]
    CreateTempFileFailed { source: io::Error },
    /// Failed to write the buffer into the temporary file.
    #[error("failed to write to a temporary file")]
    WriteFailed { source: io::Error },
    /// Failed to persist the temporary file to its final path.
    #[error("failed to persist the temporary file")]
    KeepFailed { source: PersistError },
}
