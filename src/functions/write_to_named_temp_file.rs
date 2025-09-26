use crate::{handle, map_err};
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use tempfile::{NamedTempFile, PersistError};
use thiserror::Error;

pub fn write_to_named_temp_file(buf: &[u8]) -> Result<(File, PathBuf), WriteErrorDebugToTempFileError> {
    use WriteErrorDebugToTempFileError::*;
    let mut temp = handle!(NamedTempFile::new(), CreateTempFileFailed);
    handle!(temp.write_all(buf), WriteFailed);
    map_err!(temp.keep(), KeepFailed)
}

#[derive(Error, Debug)]
pub enum WriteErrorDebugToTempFileError {
    #[error("failed to create a temporary file")]
    CreateTempFileFailed { source: io::Error },
    #[error("failed to write to a temporary file")]
    WriteFailed { source: io::Error },
    #[error("failed to persist the temporary file")]
    KeepFailed { source: PersistError },
}
