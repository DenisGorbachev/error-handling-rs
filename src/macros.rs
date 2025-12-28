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
                $($arg: $crate::_into!($arg$(: $value)?)),*
            }),
        }
    };
}

/// See also: [`handle_opt_take!`](crate::handle_opt_take)
#[macro_export]
macro_rules! handle_opt {
    ($option:expr, $variant:ident$(,)? $($arg:ident$(: $value:expr)?),*) => {
        match $option {
            Some(value) => value,
            None => return Err($variant {
                $($arg: $crate::_into!($arg$(: $value)?)),*
            }),
        }
    };
}

/// This macro is an opposite of [`handle_opt!`](crate::handle_opt) - it returns an error if the option contains a `Some` variant.
///
/// Note that this macro calls [`Option::take`], which will leave a `None` if the option was `Some(value)`.
/// Note that this macro has a mandatory argument `$some_value` (used in `if let Some($some_value) = $option.take()`), which will also be passed to the error enum variant.
#[macro_export]
macro_rules! handle_opt_take {
    ($option:expr, $variant:ident, $some_value:ident$(,)? $($arg:ident$(: $value:expr)?),*) => {
        if let Some($some_value) = $option.take() {
            return Err($variant {
                $some_value: $some_value.into(),
                $($arg: $crate::_into!($arg$(: $value)?)),*
            })
        }
    };
}

/// Returns an error when the condition is true.
///
/// This is useful for guard checks that should fail fast with a specific error variant.
#[macro_export]
macro_rules! handle_bool {
    ($condition:expr, $variant:ident$(,)? $($arg:ident$(: $value:expr)?),*) => {
        if $condition {
            return Err($variant {
                $($arg: $crate::_into!($arg$(: $value)?)),*
            });
        };
    };
}

/// Collects results from an iterator, returning a variant that wraps all errors.
///
/// `$results` must be an `impl Iterator<Item = Result<T, E>>`.
#[macro_export]
macro_rules! handle_iter {
    ($results:expr, $variant:ident$(,)? $($arg:ident$(: $value:expr)?),*) => {
        {
            let (oks, errors): (Vec<_>, Vec<_>) = itertools::Itertools::partition_result($results);
            if errors.is_empty() {
                oks
            } else {
                return Err($variant {
                    source: errors.into(),
                    $($arg: $crate::_into!($arg$(: $value)?)),*
                });
            }
        }
    };
}

/// Collects results while keeping the corresponding input items, returning `(outputs, items)` on success.
///
/// This macro returns a tuple because the iteration consumes items that may be needed later.
/// If there are no errors, `items.len() == outputs.len()`.
/// If the results iterator terminates early, the returned `items` may be shorter than the original input.
#[macro_export]
macro_rules! handle_iter_of_refs {
    ($results:expr, $items:expr, $variant:ident $(, $arg:ident$(: $value:expr)?)*) => {
        {
            let (outputs, items, errors) = std::iter::zip($results, $items).fold(
                (Vec::new(), Vec::new(), Vec::new()),
                |(mut outputs, mut items, mut errors), (result, item)| {
                    match result {
                        Ok(output) => {
                            outputs.push(output);
                            items.push(item);
                        }
                        Err(source) => {
                            errors.push($crate::ItemError {
                                item,
                                source,
                            });
                        }
                    }
                    (outputs, items, errors)
                },
            );
            if errors.is_empty() {
                (outputs, items)
            } else {
                return Err($variant {
                    source: errors.into(),
                    $($arg: $crate::_into!($arg$(: $value)?)),*
                });
            }
        }
    };
}

/// Collects results from any `IntoIterator`, wrapping all errors into one variant.
#[macro_export]
macro_rules! handle_into_iter {
    ($results:expr, $variant:ident $(, $arg:ident$(: $value:expr)?)*) => {
        $crate::handle_iter!($results.into_iter(), $variant $(, $arg$(: $value)?),*)
    };
}

/// [`handle_discard`](crate::handle_discard) should only be used when you want to discard the source error. This is discouraged. Prefer other handle-family macros that preserve the source error.
#[macro_export]
macro_rules! handle_discard {
    ($result:expr, $variant:ident$(,)? $($arg:ident$(: $value:expr)?),*) => {
        match $result {
            Ok(value) => value,
            Err(_) => return Err($variant {
                $($arg: $crate::_into!($arg$(: $value)?)),*
            }),
        }
    };
}

/// [`map_err`](crate::map_err) should be used only when the error variant doesn't capture any owned variables (which is very rare), or exactly at the end of the block (in the position of returned expression).
#[macro_export]
macro_rules! map_err {
    ($result:expr, $variant:ident$(,)? $($arg:ident$(: $value:expr)?),*) => {
        $result.map_err(|source| $variant {
            source: source.into(),
            $($arg: $crate::_into!($arg$(: $value)?)),*
        })
    };
}

/// Internal
#[doc(hidden)]
#[macro_export]
macro_rules! _into {
    ($arg:ident) => {
        $arg.into()
    };
    ($arg:ident: $value:expr) => {
        $value.into()
    };
}

/// Internal
#[doc(hidden)]
#[macro_export]
macro_rules! _index_err {
    ($f:ident) => {
        |(index, item)| $f(item).map_err(|err| (index, err))
    };
}

/// Internal
#[doc(hidden)]
#[macro_export]
macro_rules! _index_err_async {
    ($f:ident) => {
        async |(index, item)| $f(item).await.map_err(|err| (index, err))
    };
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use crate::{ErrVec, PathBufDisplay};
    use futures::future::join_all;
    use serde::{Deserialize, Serialize};
    use std::io;
    use std::path::{Path, PathBuf};
    use std::str::FromStr;
    use std::sync::{Arc, RwLock};
    use thiserror::Error;
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
            let config = handle!(parse_config(&dir, format).await, ParseConfigFailed);
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
            .map(check_file)
            .collect::<JoinSet<_>>()
            .join_all()
            .await;
        Ok(handle_into_iter!(results, CheckFileFailed))
    }

    #[allow(dead_code)]
    async fn read_files_ref(paths: Vec<PathBuf>) -> Result<Vec<String>, ReadFilesRefError> {
        use ReadFilesRefError::*;
        let iter = paths.iter().map(check_file_ref);
        let results = join_all(iter).await;
        let items = paths.into_iter().map(PathBufDisplay::from);
        let (outputs, _items) = handle_iter_of_refs!(results.into_iter(), items, CheckFileRefFailed);
        Ok(outputs)
    }

    // async fn check_file(path: &Path)

    /// This function exists to test error handling in async code
    #[allow(dead_code)]
    async fn process(number: u32) -> Result<u32, ProcessError> {
        Ok(number)
    }

    #[derive(Error, Debug)]
    enum PrintNameCommandError {
        #[error("failed to parse config")]
        ParseConfigFailed { source: ParseConfigError },
    }

    /// Variants don't have the `format` field because every variant already corresponds to a single specific format
    /// Some variants have the `path` field because the `contents` depends on `path`
    /// Some `source` field types are wrapped in `Box` according to suggestion from `result_large_err` lint
    #[derive(Error, Debug)]
    enum ParseConfigError {
        #[error("failed to read the file: {path}")]
        ReadFileFailed { path: PathBuf, source: io::Error },
        #[error("failed to deserialize the file contents from JSON: {path}")]
        DeserializeFromJson { path: PathBuf, contents: String, source: Box<serde_json::error::Error> },
        #[error("failed to deserialize the file contents from TOML: {path}")]
        DeserializeFromToml { path: PathBuf, contents: String, source: Box<toml::de::Error> },
    }

    #[allow(dead_code)]
    #[derive(Error, Debug)]
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

    #[derive(Error, Debug)]
    enum ParseEvenNumberError {
        #[error("failed to parse input")]
        InputParseFailed { source: <u32 as FromStr>::Err },
        #[error("number is not even: {number}")]
        NumberNotEven { number: u32 },
    }

    #[derive(Error, Debug)]
    enum FindEvenError {
        #[error("even number not found")]
        NotFound,
    }

    #[derive(Error, Debug)]
    enum MultiplyEvensError {
        #[error("failed to check {len} numbers", len = source.len())]
        CheckEvensFailed { source: ErrVec },
    }

    #[derive(Error, Debug)]
    enum ReadFilesError {
        #[error("failed to check {len} files", len = source.len())]
        CheckFileFailed { source: ErrVec },
    }

    #[derive(Error, Debug)]
    enum ReadFilesRefError {
        #[error("failed to check {len} files", len = source.len())]
        CheckFileRefFailed { source: ErrVec },
    }

    #[derive(Error, Debug)]
    enum CheckEvenError {
        #[error("number is not even: {number}")]
        NumberNotEven { number: u32 },
    }

    async fn check_file(path: PathBuf) -> Result<String, CheckFileError> {
        use CheckFileError::*;
        let content = handle!(read_to_string(&path).await, ReadToStringFailed, path);
        handle_bool!(content.is_empty(), FileIsEmpty, path);
        Ok(content)
    }

    #[derive(Error, Debug)]
    enum CheckFileError {
        #[error("failed to read the file to string: {path}")]
        ReadToStringFailed { path: PathBuf, source: io::Error },
        #[error("file is empty: {path}")]
        FileIsEmpty { path: PathBuf },
    }

    async fn check_file_ref(path: &PathBuf) -> Result<String, CheckFileRefError> {
        use CheckFileRefError::*;
        let content = handle!(read_to_string(&path).await, ReadToStringFailed);
        handle_bool!(content.is_empty(), FileIsEmpty);
        Ok(content)
    }

    #[derive(Error, Debug)]
    enum CheckFileRefError {
        #[error("failed to read the file to string")]
        ReadToStringFailed { source: io::Error },
        #[error("file is empty")]
        FileIsEmpty,
    }

    #[derive(Clone, Debug)]
    struct Db {
        user: User,
    }

    #[derive(Clone, Debug)]
    struct User {
        username: String,
    }

    #[allow(dead_code)]
    fn get_username(db: Arc<RwLock<Db>>) -> Result<String, GetUsernameError> {
        use GetUsernameError::*;
        // `db.read()` returns `LockResult` whose Err variant is `PoisonError<RwLockReadGuard<'_, T>>`, which contains an anonymous lifetime
        // The error enum returned from this function must contain only owned fields, so it can't contain a `source` that has a lifetime
        // Therefore, we have to use handle_discard!, although it is discouraged
        let guard = handle_discard!(db.read(), AcquireReadLockFailed);
        let username = guard.user.username.clone();
        Ok(username)
    }

    #[derive(Error, Debug)]
    pub enum GetUsernameError {
        #[error("failed to acquire read lock")]
        AcquireReadLockFailed,
    }

    #[allow(dead_code)]
    fn get_answer(prompt: String, get_response: &mut impl FnMut(String) -> Result<WeirdResponse, io::Error>) -> Result<String, GetAnswerError> {
        use GetAnswerError::*;
        // Since the `get_response` external API doesn't return the `prompt` in its error, we have to clone `prompt` before passing it as argument, so that we could pass it to the error enum variant
        // Cloning may be necessary with external APIs that don't return arguments in errors, but it must not be necessary in our code
        let mut response = handle!(get_response(prompt.clone()), GetResponseFailed, prompt);
        handle_opt_take!(response.error, ResponseContainsError, error);
        Ok(response.answer)
    }

    /// OpenAI Responses API returns a response with `error: Option<WeirdResponseError>` field, which is weird, but must still be handled
    #[derive(Debug)]
    pub struct WeirdResponse {
        answer: String,
        error: Option<WeirdResponseError>,
    }

    #[allow(dead_code)]
    #[derive(Error, Debug)]
    pub enum WeirdResponseError {
        #[error("prompt is empty")]
        PromptIsEmpty,
        #[error("context limit reached")]
        ContextLimitReached,
    }

    /// [`GetAnswerError::GetResponseFailed`] `error` attribute doesn't contain a reference to `{prompt}` because the prompt can be very long, so it would make the error message very long, which is undesirable
    #[derive(Error, Debug)]
    pub enum GetAnswerError {
        #[error("failed to get response")]
        GetResponseFailed { source: io::Error, prompt: String },
        #[error("response contains an error")]
        ResponseContainsError { error: WeirdResponseError },
    }
}
