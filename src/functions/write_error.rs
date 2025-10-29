use crate::functions::write_to_named_temp_file;
use std::error::Error;
use std::io;
use std::io::{Write, stderr};

pub fn write_error(error: &dyn Error, writer: &mut impl Write) -> Result<(), io::Error> {
    writeln!(writer, "- {}", error)?;
    let mut source = error;
    while let Some(source_new) = source.source() {
        writeln!(writer, "- {}", source_new)?;
        source = source_new;
    }
    writeln!(writer)?;
    let error_debug = format!("{:#?}", error);
    let result = write_to_named_temp_file::write_to_named_temp_file(error_debug.as_bytes());
    match result {
        Ok((_file, path_buf)) => {
            writeln!(writer, "See the full error report:\nless {}", path_buf.display())
        }
        Err(other_error) => {
            writeln!(writer, "{other_error:#?}")
        }
    }
}

pub fn eprintln_error(error: &dyn Error) {
    let mut stderr = stderr().lock();
    let result = write_error(error, &mut stderr);
    match result {
        Ok(()) => (),
        Err(err) => eprintln!("failed to write to stderr: {:#?}", err),
    }
}

#[cfg(test)]
mod tests {
    use crate::functions::write_error::tests::JsonSchemaNewError::InputMustBeObject;
    use crate::write_error;
    use thiserror::Error;

    #[ignore]
    #[test]
    fn must_write_error() {
        let error = CliRunError::CommandRunFailed {
            source: CommandRunError::I18nUpdateRunFailed {
                source: I18nUpdateRunError::UpdateRowsFailed {
                    sources: vec![
                        UpdateRowError::I18nRequestFailed {
                            source: I18nRequestError::JsonSchemaNewFailed {
                                source: InputMustBeObject {
                                    input: "foo".to_string(),
                                },
                            },
                            row: Row::new("Foo"),
                        },
                        UpdateRowError::I18nRequestFailed {
                            source: I18nRequestError::RequestSendFailed {
                                source: tokio::io::Error::new(tokio::io::ErrorKind::AddrNotAvailable, "Address 239.143.73.1 is not available"),
                            },
                            row: Row::new("Bar"),
                        },
                    ],
                },
            },
        };
        let mut output = Vec::new();
        write_error(&error, &mut output).unwrap();
        let string = String::from_utf8(output).unwrap();
        assert_eq!(string, include_str!("write_error/must_write_error.txt"))
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
        #[error("failed to update {len} rows", len = sources.len())]
        UpdateRowsFailed { sources: Vec<UpdateRowError> },
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
        #[error("input must be an object")]
        InputMustBeObject { input: String },
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
