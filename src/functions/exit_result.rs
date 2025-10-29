use crate::eprintln_error;
use std::error::Error;
use std::process::ExitCode;

pub fn exit_result<E: Error + 'static>(result: Result<(), E>) -> ExitCode {
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln_error(&err);
            ExitCode::FAILURE
        }
    }
}
