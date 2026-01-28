use crate::eprintln_error;
use std::error::Error;
use std::process::ExitCode;

#[cfg(feature = "futures")]
use futures::Stream;
#[cfg(feature = "futures")]
use futures::StreamExt;
#[cfg(feature = "futures")]
use std::pin::pin;

/// Converts a [`Result`] into an [`ExitCode`], printing a detailed error trace on failure.
pub fn exit_result<E: Error>(result: Result<ExitCode, E>) -> ExitCode {
    result.unwrap_or_else(|err| {
        eprintln_error(&err);
        ExitCode::FAILURE
    })
}

/// Converts an [`impl IntoIterator<Item = Result<(), E>>`](IntoIterator) into an [`ExitCode`], printing a detailed error trace on the first failure.
pub fn exit_iterator_of_results_print_first<E: Error>(iter: impl IntoIterator<Item = Result<(), E>>) -> ExitCode {
    for result in iter.into_iter() {
        if let Err(error) = result {
            eprintln_error(&error);
            return ExitCode::FAILURE;
        }
    }
    ExitCode::SUCCESS
}

#[cfg(feature = "futures")]
/// Converts an [`impl IntoIterator<Item = Result<(), E>>`](IntoIterator) into an [`ExitCode`], printing a detailed error trace on the first failure.
pub async fn exit_stream_of_results_print_first<E: Error>(stream: impl Stream<Item = Result<(), E>>) -> ExitCode {
    let mut stream = pin!(stream);
    if let Some(Err(error)) = stream.next().await {
        eprintln_error(&error);
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}
