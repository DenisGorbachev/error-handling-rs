use crate::eprintln_error;
use std::error::Error;
use std::process::ExitCode;

/// Converts a [`Result`] into an [`ExitCode`], printing a detailed error trace on failure.
pub fn exit_result<E: Error>(result: Result<(), E>) -> ExitCode {
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln_error(&err);
            ExitCode::FAILURE
        }
    }
}

/// Converts an [`impl IntoIterator<Item = Result<(), E>>`](IntoIterator) into an [`ExitCode`], printing a detailed error trace on the first failure.
pub fn exit_iterator_of_results<E: Error>(iter: impl IntoIterator<Item = Result<(), E>>) -> ExitCode {
    for result in iter.into_iter() {
        if let Err(error) = result {
            eprintln_error(&error);
            return ExitCode::FAILURE;
        }
    }
    ExitCode::SUCCESS
}
