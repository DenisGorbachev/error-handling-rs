/// [`handle!`](crate::handle) is a better alternative to [`map_err`](Result::map_err) because it doesn't capture any variables from the environment if the result is [`Ok`], only when the result is [`Err`].
/// By contrast, a closure passed to `map_err` always captures the variables from environment, regardless of whether the result is [`Ok`] or [`Err`]
/// Use [`handle!`](crate::handle) if you need to pass owned variables to an error variant (which is returned only in case when result is [`Err`])
/// In addition, this macro captures the original error in the `source` variable, and sets it as the `source` key of the error variant
///
/// Note: [`handle!`](crate::handle) assumes that your error variant is a struct variant
#[macro_export]
macro_rules! handle {
    ($result:expr, $variant:ident$(,)? $($arg:ident$(: $value:expr)?),*) => {
        match $result {
            Ok(value) => value,
            Err(source) => return Err($variant {
                source: source.into(),
                $($arg: $crate::into!($arg$(: $value)?)),*
            }),
        }
    };
}

/// `$results` must be an `impl Iterator<Item = Result<T, E>>`
#[macro_export]
macro_rules! handle_iter {
    ($results:expr, $variant:ident$(,)? $($arg:ident$(: $value:expr)?),*) => {
        {
            let (oks, errors): (Vec<_>, Vec<_>) = itertools::Itertools::partition_result($results);
            if errors.is_empty() {
                oks
            } else {
                return Err($variant {
                    sources: errors.into(),
                    $($arg: $crate::into!($arg$(: $value)?)),*
                });
            }
        }
    };
}

/// `$results` must be an `impl IntoIterator<Item = Result<T, E>>`
#[macro_export]
macro_rules! handle_into_iter {
    ($results:expr, $variant:ident) => {
        $crate::handle_iter!($results.into_iter(), $variant)
    };
    ($results:expr, $variant:ident, $($arg:ident$(: $value:expr)?),*) => {
        $crate::handle_iter!($results.into_iter(), $variant, $($arg$(: $value)?),*)
    };
}

/// [`handle_map_err!`](crate::handle_map_err) should be used only when the error variant doesn't capture any owned variables (which is very rare), or exactly at the end of the block (in the position of returned expression).
///
/// Use [`handle`](crate::handle) if the error variant does capture some owned variables.
#[macro_export]
macro_rules! handle_map_err {
    ($result:expr, $variant:ident$(,)? $($arg:ident$(: $value:expr)?),*) => {
        $result.map_err(|source| $variant {
            source: source.into(),
            $($arg: $crate::into!($arg$(: $value)?)),*
        })?
    };
}

/// [`handle_final!`](crate::handle_final) should be used only at the end of the block (in the position of returned expression).
///
/// Use [`handle`](crate::handle) in the middle of the block.
///
/// [`handle_final!`](crate::handle_final) is different from [`handle_map_err!`](crate::handle_map_err) in that it doesn't apply the `?` operator to the resulting expression (it returns a `Result<T, E>`, not just `T`)
#[macro_export]
macro_rules! handle_final {
    ($result:expr, $variant:ident$(,)? $($arg:ident$(: $value:expr)?),*) => {
        $result.map_err(|source| $variant {
            source: source.into(),
            $($arg: $crate::into!($arg$(: $value)?)),*
        })
    };
}

#[macro_export]
macro_rules! handle_direct {
    ($result:expr, $source:ident, $error:expr) => {
        match $result {
            Ok(value) => value,
            Err($source) => return Err($error),
        }
    };
}

#[macro_export]
macro_rules! handle_opt {
    ($option:expr, $variant:ident$(,)? $($arg:ident$(: $value:expr)?),*) => {
        match $option {
            Some(value) => value,
            None => return Err($variant {
                $($arg: $crate::into!($arg$(: $value)?)),*
            }),
        }
    };
}

#[macro_export]
macro_rules! handle_bool {
    ($condition:expr, $variant:ident$(,)? $($arg:ident$(: $value:expr)?),*) => {
        if $condition {
            return Err($variant {
                $($arg: $crate::into!($arg$(: $value)?)),*
            });
        };
    };
}

#[macro_export]
macro_rules! into {
    ($arg:ident) => {
        $arg.into()
    };
    ($arg:ident: $value:expr) => {
        $value.into()
    };
}

#[cfg(test)]
mod tests {
    use crate::{Display, Error, PathBufDisplay};
    use serde::{Deserialize, Serialize};
    use std::io;
    use std::path::{Path, PathBuf};
    use std::str::FromStr;
    use tokio::fs::read_to_string;
    use tokio::task::JoinSet;

    #[allow(dead_code)]
    struct PrintNameCommand {
        dir: PathBuf,
        format: Format,
    }

    #[allow(dead_code)]
    impl PrintNameCommand {
        async fn run(self) -> Result<(), PrintNameCommandError> {
            use PrintNameCommandError::*;
            let Self {
                dir,
                format,
            } = self;
            let config = handle_map_err!(parse_config(&dir, format).await, ParseConfigFailed);
            println!("{}", config.name);
            Ok(())
        }
    }

    /// This function tests the [`crate::handle!`] macro
    #[allow(dead_code)]
    async fn parse_config(dir: &Path, format: Format) -> Result<Config, ParseConfigError> {
        use Format::*;
        use ParseConfigError::*;
        let path_buf = dir.join("config.json");
        let contents = handle!(read_to_string(&path_buf).await, ReadFileFailed, path: path_buf);
        match format {
            Json => {
                let config = handle!(serde_json::de::from_str(&contents), DeserializeFromJson, path: path_buf, contents);
                Ok(config)
            }
            Toml => {
                let config = handle!(toml::de::from_str(&contents), DeserializeFromToml, path: path_buf, contents);
                Ok(config)
            }
        }
    }

    /// This function tests the [`crate::handle_opt!`] macro
    #[allow(dead_code)]
    fn find_even(numbers: Vec<u32>) -> Result<u32, FindEvenError> {
        use FindEvenError::*;
        let even = handle_opt!(numbers.iter().find(|x| *x % 2 == 0), NotFound);
        Ok(*even)
    }

    /// This function tests the [`crate::handle_iter!`] macro
    #[allow(dead_code)]
    fn multiply_evens(numbers: Vec<u32>) -> Result<Vec<u32>, MultiplyEvensError> {
        use MultiplyEvensError::*;
        let results = numbers.into_iter().map(|number| {
            use CheckEvenError::*;
            if number % 2 == 0 {
                Ok(number * 10)
            } else {
                Err(NumberNotEven {
                    number,
                })
            }
        });
        Ok(handle_iter!(results, CheckEvensFailed))
    }

    /// This function tests the [`crate::handle_into_iter!`] macro
    #[allow(dead_code)]
    async fn read_files(paths: Vec<PathBuf>) -> Result<Vec<String>, ReadFilesError> {
        use ReadFilesError::*;
        let results = paths
            .into_iter()
            .enumerate()
            .map(async |(index, path)| {
                use CheckFileError::*;
                let content = handle!(read_to_string(&path).await, ReadToStringFailed, index);
                handle_bool!(content.is_empty(), FileIsEmpty, index);
                Ok(content)
            })
            .collect::<JoinSet<_>>()
            .join_all()
            .await;
        Ok(handle_into_iter!(results, CheckFileFailed))
    }

    // async fn check_file(path: &Path)

    /// This function exists to test error handling in async code
    #[allow(dead_code)]
    async fn process(number: u32) -> Result<u32, ProcessError> {
        Ok(number)
    }

    #[derive(Error, Display, Debug)]
    enum PrintNameCommandError {
        ParseConfigFailed { source: ParseConfigError },
    }

    /// Variants don't have the `format` field because every variant already corresponds to a single specific format
    /// Some variants have the `path` field because the `contents` depends on `path`
    /// `path` has type `PathBufDisplay` because `PathBuf` doesn't implement `Display`
    /// Some `source` field types are wrapped in `Box` according to suggestion from `result_large_err` lint
    #[derive(Error, Display, Debug)]
    enum ParseConfigError {
        ReadFileFailed { path: PathBufDisplay, source: io::Error },
        DeserializeFromJson { path: PathBufDisplay, contents: String, source: Box<serde_json::error::Error> },
        DeserializeFromToml { path: PathBufDisplay, contents: String, source: Box<toml::de::Error> },
    }

    #[allow(dead_code)]
    #[derive(Error, Display, Debug)]
    enum ProcessError {}

    #[allow(dead_code)]
    #[derive(Copy, Clone, Debug)]
    enum Format {
        Json,
        Toml,
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    struct Config {
        name: String,
        timeout: u64,
        parallel: bool,
    }

    #[allow(dead_code)]
    fn parse_even_number(input: &str) -> Result<u32, ParseEvenNumberError> {
        use ParseEvenNumberError::*;
        let number = handle!(input.parse::<u32>(), InputParseFailed);
        handle_bool!(number % 2 != 0, NumberNotEven, number);
        Ok(number)
    }

    #[derive(Error, Display, Debug)]
    enum ParseEvenNumberError {
        InputParseFailed { source: <u32 as FromStr>::Err },
        NumberNotEven { number: u32 },
    }

    #[derive(Error, Display, Debug)]
    enum FindEvenError {
        NotFound,
    }

    #[derive(Error, Display, Debug)]
    enum MultiplyEvensError {
        CheckEvensFailed { sources: Vec<CheckEvenError> },
    }

    #[derive(Error, Display, Debug)]
    enum ReadFilesError {
        CheckFileFailed { sources: Vec<CheckFileError> },
    }

    #[derive(Error, Display, Debug)]
    enum CheckEvenError {
        NumberNotEven { number: u32 },
    }

    #[derive(Error, Display, Debug)]
    enum CheckFileError {
        ReadToStringFailed { index: usize, source: io::Error },
        FileIsEmpty { index: usize },
    }
}
