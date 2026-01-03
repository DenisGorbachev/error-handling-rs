use crate::write_to_named_temp_file;
use core::error::Error;
use core::fmt::Formatter;
use std::io;
use std::io::{Write, stderr};

/// Writes a human-readable error trace to the provided formatter.
pub fn writeln_error_to_formatter<E: Error + ?Sized>(error: &E, f: &mut Formatter<'_>) -> core::fmt::Result {
    use std::fmt::Write;
    write!(f, "- {error}")?;
    if let Some(source_new) = error.source() {
        f.write_char('\n')?;
        writeln_error_to_formatter(source_new, f)
    } else {
        Ok(())
    }
}

/// Writes a human-readable error trace to the provided writer and persists the full debug output to a temp file.
///
/// This is useful for CLI tools that want a concise error trace on stderr and a path to a full report.
pub fn writeln_error_to_writer_and_file<E: Error>(error: &E, writer: &mut dyn Write) -> Result<(), io::Error> {
    writeln!(writer, "{error}")?;
    writeln!(writer)?;
    let error_debug = format!("{error:#?}");
    let result = write_to_named_temp_file(error_debug.as_bytes());
    match result {
        Ok((_file, path_buf)) => {
            writeln!(writer, "See the full error report:\nless {}", path_buf.display())
        }
        Err(other_error) => {
            writeln!(writer, "{other_error:#?}")
        }
    }
}

/// Writes an error trace to stderr and, if possible, includes a path to the full error report.
pub fn eprintln_error<E>(error: &E)
where
    E: Error + 'static,
{
    let mut stderr = stderr().lock();
    let result = writeln_error_to_writer_and_file(error, &mut stderr);
    match result {
        Ok(()) => (),
        Err(err) => eprintln!("failed to write to stderr: {err:#?}"),
    }
}

#[cfg(test)]
mod tests {
    use crate::functions::writeln_error::tests::JsonSchemaNewError::{InvalidInput, InvalidValues};
    use crate::{ErrVec, ErrorDisplayer};
    use CliRunError::*;
    use CommandRunError::*;
    use I18nRequestError::*;
    use I18nUpdateRunError::*;
    use JsonValueNewError::*;
    use UpdateRowError::*;
    use pretty_assertions::assert_eq;
    use std::error::Error;
    use thiserror::Error;

    #[test]
    fn must_write_error() {
        let error = CommandRunFailed {
            source: I18nUpdateRunFailed {
                source: UpdateRowsFailed {
                    source: vec![
                        I18nRequestFailed {
                            source: JsonSchemaNewFailed {
                                source: InvalidInput {
                                    input: "foo".to_string(),
                                },
                            },
                            row: Row::new("Foo"),
                        },
                        I18nRequestFailed {
                            source: RequestSendFailed {
                                source: tokio::io::Error::new(tokio::io::ErrorKind::AddrNotAvailable, "server at 239.143.73.1 did not respond"),
                            },
                            row: Row::new("Bar"),
                        },
                    ]
                    .into(),
                },
            },
        };
        let expected = include_str!("writeln_error/fixtures/must_write_error.txt");
        assert_write_eq(&error, expected);
    }

    #[test]
    fn must_write_nested_error() {
        let error = UpdateRowsFailed {
            source: vec![I18nRequestFailed {
                source: JsonSchemaNewFailed {
                    source: InvalidValues {
                        source: vec![
                            InvalidKey {
                                key: "zed".to_string(),
                            },
                            InvalidKey {
                                key: "moo".to_string(),
                            },
                        ]
                        .into(),
                    },
                },
                row: Row::new("Foo"),
            }]
            .into(),
        };
        let expected = include_str!("writeln_error/fixtures/must_write_nested_error.txt");
        assert_write_eq(&error, expected);
    }

    fn assert_write_eq<E: Error>(error: &E, expected: &str) {
        use std::fmt::Write;
        let mut actual = String::new();
        let displayer = ErrorDisplayer(error);
        writeln!(actual, "{displayer}").unwrap();
        eprintln!("{}", &actual);
        assert_eq!(actual, expected)
    }

    #[derive(Error, Debug)]
    pub enum CliRunError {
        #[error("failed to run CLI command")]
        CommandRunFailed { source: CommandRunError },
    }

    #[derive(Error, Debug)]
    pub enum CommandRunError {
        #[error("failed to run i18n update command")]
        I18nUpdateRunFailed { source: I18nUpdateRunError },
    }

    #[derive(Error, Debug)]
    pub enum I18nUpdateRunError {
        #[error("failed to update {len} rows", len = source.len())]
        UpdateRowsFailed { source: ErrVec<UpdateRowError> },
    }

    #[derive(Error, Debug)]
    pub enum UpdateRowError {
        #[error("failed to send an i18n request for row '{row}'", row = row.name)]
        I18nRequestFailed { source: I18nRequestError, row: Row },
    }

    #[derive(Error, Debug)]
    pub enum I18nRequestError {
        #[error("failed to construct a JSON schema")]
        JsonSchemaNewFailed { source: JsonSchemaNewError },
        #[error("failed to send a request")]
        RequestSendFailed { source: tokio::io::Error },
    }

    #[derive(Error, Debug)]
    pub enum JsonSchemaNewError {
        #[error("input must be a JSON object")]
        InvalidInput { input: String },
        #[error("failed to construct {len} values", len = source.len())]
        InvalidValues { source: ErrVec<JsonValueNewError> },
    }

    #[derive(Error, Debug)]
    pub enum JsonValueNewError {
        #[error("'{key}' must be a JSON value")]
        InvalidKey { key: String },
    }

    #[derive(Debug)]
    pub struct Row {
        name: String,
    }

    impl Row {
        pub fn new(name: impl Into<String>) -> Self {
            Self {
                name: name.into(),
            }
        }
    }
}
