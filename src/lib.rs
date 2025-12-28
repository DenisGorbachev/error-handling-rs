//! Macros for ergonomic error handling with [thiserror](https://crates.io/crates/thiserror).
//!
//! # Example
//!
//! ```rust
//! # #[cfg(feature = "std")]
//! # {
//! # use std::io;
//! # use std::fs::read_to_string;
//! # use std::path::{Path, PathBuf};
//! # use serde::{Deserialize, Serialize};
//! # use serde_json::from_str;
//! # use thiserror::Error;
//! # use errgonomic::handle;
//! #
//! #[derive(Serialize, Deserialize)]
//! struct Config {/* some fields */}
//!
//! // bad: doesn't return the path to config (the user won't be able to fix it)
//! fn parse_config_v1(path: PathBuf) -> io::Result<Config> {
//!     let contents = read_to_string(&path)?;
//!     let config = from_str(&contents).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
//!     Ok(config)
//! }
//!
//! // good: returns the path to config & the underlying deserialization error (the user will be able fix it)
//! fn parse_config_v2(path: PathBuf) -> Result<Config, ParseConfigError> {
//!     use ParseConfigError::*;
//!     let contents = handle!(read_to_string(&path), ReadToStringFailed, path);
//!     let config = handle!(from_str(&contents), DeserializeFailed, path, contents);
//!     Ok(config)
//! }
//!
//! #[derive(Error, Debug)]
//! enum ParseConfigError {
//!     #[error("failed to read file to string: '{}'", path.display())]
//!     ReadToStringFailed { path: PathBuf, source: std::io::Error },
//!     #[error("failed to parse the file contents into config: '{}'", path.display())]
//!     DeserializeFailed { path: PathBuf, contents: String, source: serde_json::Error }
//! }
//! # }
//! ```
//!
//! Advantages:
//!
//! * `parse_config_v2` allows you to determine exactly what error has occurred
//! * `parse_config_v2` provides you with all information needed to fix the underlying issue
//! * `parse_config_v2` allows you to retry the call by reusing the `path` (avoiding unnecessary clones)
//!
//! Disadvantages:
//!
//! * `parse_config_v2` is longer
//!
//! That means `parse_config_v2` is strictly better but requires more code. However, with LLMs, writing more code is not an issue. Therefore, it's better to use a more verbose approach, which provides you with better errors.
//!
//! This crates provides the `handle` family of macros to simplify the error handling code.
//!
//! # Better debugging
//!
//! To improve your debugging experience: call [`exit_result`] in `main` right before return, and it will display all information necessary to understand the root cause of the error (see example below).
//!
//! ```rust
//! # #[cfg(feature = "std")]
//! # {
//! # use errgonomic::exit_result;
//! # use thiserror::Error;
//! # use std::process::ExitCode;
//! #
//! # #[derive(Error, Debug)]
//! # enum Err {}
//! #
//! # fn run() -> Result<(), Err> { Ok(()) }
//! #
//! pub fn main() -> ExitCode {
//!     exit_result(run())
//! }
//! # }
//! ```
//!
//! This will produce a nice "error trace" like below:
#![doc = "```text"]
#![doc = include_str!("./functions/writeln_error/fixtures/must_write_error.txt")]
#![doc = "```"]
//!
//! # Better error handling
//!
//! **Goal**: Help the caller diagnose the issue, fix it, and retry the call.
//!
//! **Approach**: Every error must be represented by a unique enum variant with relevant fields.
//!
//! ## Guidelines
//!
//! * Every error type must be an enum
//! * Every error enum variant must be a struct variant
//! * Every error enum variant must contain one field per owned variable that is relevant to the fallible expression that this variant wraps
//!   * The relevant variable is a variable whose value determines whether the fallible expression returns an [`Ok`] or an [`Err`]
//! * Every error enum variant must have fields only for [`data types`](#data-type), not for [`non-data types`](#non-data-type)
//! * Every error enum variant field must have an owned type (not a reference)
//! * Every error enum should be located below the function that returns it (in the same file)
//! * Every fallible function must return a unique error type
//! * Every call to another fallible function must be wrapped in a unique error enum variant
//! * If the function contains only one fallible expression, this expression must still be wrapped in an error enum variant
//! * Every variable that contains secret data (the one which must not be displayed or logged, e.g. password, API key, personally identifying information) must have a type that doesn't output the underlying data in the Debug and Display impls (e.g. [`secrecy::SecretBox`](https://docs.rs/secrecy/latest/secrecy/struct.SecretBox.html))
//! * The code that calls a fallible function on each element of a collection should return an `impl Iterator<Item = Result<T, E>>` instead of short-circuiting on the first error
//! * If Clippy outputs a `result_large_err` warning, then the large fields of the error enum must be wrapped in a `Box`
//! * If the error enum variant has a `source` field, then this field must be the first field
//! * The code must not use strings for error messages
//! * The production code must not use `unwrap` or `expect` (only tests may use `unwrap` or `expect`)
//! * If each field of each variant of the error enum implements `Copy`, then the error enum must implement `Copy` too
//! * If an argument of callee implements `Copy`, the callee must not include it in the list of error enum variant fields (the caller must include it because of the rule to include all relevant owned variables)
//!
//! ## Conveniences
//!
//! * Every fallible function body must begin with `use ThisFunctionError::*;`, where `ThisFunctionError` must be the name of this function's error enum (for example: `use ParseConfigError::*;`)
//! * The error handling code must use the error enum variant names without the error enum name prefix (for example: `ReadFileFailed` instead of `ParseConfigError::ReadFileFailed`)
//!
//! ## Naming
//!
//! * The name of the error enum must end with `Error` (for example: `ParseConfigError`)
//! * The name of the error enum variant should end with `Failed` or `NotFound` or `Invalid` (for example: `ReadFileFailed`, `UserNotFound`, `PasswordInvalid`)
//! * If the error variant name is associated with a child function call, the name of the error variant must be equal to the name of the function converted to CamelCase concatenated with `Failed` (for example: if the parent function calls `read_file`, then it should call it like this: `handle!(read_file(&path), ReadFileFailed, path)`
//! * The name of the error enum must include the name of the function converted to CamelCase
//!   * If the function is a freestanding function, the name of the error type must be exactly equal to the name of the function converted to CamelCase concatenated with `Error`
//!   * If the function is an associated function, the name of the error type must be exactly equal to the name of the type without generics concatenated with the name of the function in CamelCase concatenated with `Error`
//!   * If the error is specified as an associated type of a foreign trait with multiple functions that return this associated error type, then the name of the error type must be exactly equal to the name of the trait including generics concatenated with the name of the type for which this trait is implemented concatenated with `Error`
//! * If the error enum is defined for a `TryFrom<A> for B` impl, then its name must be equal to "Convert{A}To{B}Error"
//!
//! # Macros
//!
//! Use the following macros for more concise error handling:
//!
//! * [`handle!`] instead of [`Result::map_err`]
//! * [`handle_opt!`] instead of [`Option::ok_or`] and [`Option::ok_or_else`]
//! * [`handle_bool!`] instead of `if condition { return Err(...) }`
//! * [`handle_iter!`] instead of code that handles errors in iterators
//! * [`handle_iter_of_refs!`] instead of the code handles errors in iterators of references (where the values are still being owned by the underlying collection)
//! * [`handle_into_iter!`] replaces the code that handles errors in collections that implement [`IntoIterator`] (including [`Vec`] and [`HashMap`](std::collections::HashMap)
//!
//! # Definitions
//!
//! ## Fallible expression
//!
//! An expression that returns a [`Result`].
//!
//! ## Data type
//!
//! A type that holds the actual data.
//!
//! For example:
//!
//! * `bool`
//! * `String`
//! * `PathBuf`
//!
//! ## Non-data type
//!
//! A type that doesn't hold the actual data.
//!
//! For example:
//!
//! * `RestClient` doesn't point to the actual data, it only allows querying it.
//! * `DatabaseConnection` doesn't hold the actual data, it only allows querying it.
//!

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
extern crate core;

mod macros;

mod types;

pub use types::*;

mod functions;

pub use functions::*;

#[cfg(all(test, feature = "std"))]
mod drafts;
