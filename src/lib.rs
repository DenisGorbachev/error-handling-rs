//! # Error handling
//!
//! ## Goal
//!
//! Help the caller diagnose the issue, fix it, and retry the call.
//!
//! ## Approach
//!
//! Every error must be represented by a unique enum variant with relevant fields.
//!
//! ## Guidelines
//!
//! * Every fallible function must return a unique error type
//! * Every error type must be an enum
//! * Every error enum variant must be a struct variant
//! * Every error enum variant field must have an owned type (not a reference)
//! * Every error enum variant must contain one field per owned variable that is relevant to the fallible expression that this variant wraps
//!   * The relevant variable is a variable whose value determines whether the fallible expression returns an [`Ok`] or an [`Err`]
//! * Every error enum must have a `#[derive(Error, Display, Debug)]` attribute
//!   * `use derive_more::Error;`
//!   * `use fmt_derive::Display;`
//! * Every error enum should be located below the function that returns it (in the same file)
//! * The name of the error enum must end with `Error` (for example: `ParseConfigError`)
//! * The name of the error enum must include the name of the function converted to CamelCase
//!   * If the function is a freestanding function, the name of the error type must be exactly equal to the name of the function converted to CamelCase concatenated with `Error`
//!   * If the function is an associated function, the name of the error type must be exactly equal to the name of the type without generics concatenated with the name of the function in CamelCase concatenated with `Error`
//!   * If the error is specified as an associated type of a foreign trait with multiple functions that return this associated error type, then the name of the error type must be exactly equal to the name of the trait including generics concatenated with the name of the type for which this trait is implemented concatenated with `Error`
//! * The name of the error enum variant should end with `Failed` (for example: `ReadFileFailed`)
//! * Every call to another fallible function must be wrapped in a unique error enum variant
//! * If the function contains only one fallible expression, this expression must still be wrapped in an error enum variant
//! * Every variable that contains secret data (the one which must not be displayed or logged, e.g. password, API key, personally identifying information) must have a type that doesn't output the underlying data in the Debug and Display impls (e.g. [`secrecy::SecretBox`](https://docs.rs/secrecy/latest/secrecy/struct.SecretBox.html))
//! * The code that calls a fallible function on each element of a collection should return an `impl Iterator<Item = Result<T, E>>` instead of short-circuiting on the first error
//! * If Clippy outputs a `result_large_err` warning, then the large fields of the error enum must be wrapped in a `Box`
//! * Every error enum variant must have a `#[display(...)]` attribute
//! * If the error enum variant has a `source` field, then the first argument of `#[display(...)]` attribute must end with "\n{source}"
//! * If the error enum variant has a `source` field, then this field must be the first field
//! * If the error type is defined for a `TryFrom<A> for B` impl, then its name must be equal to "Convert{A}To{B}Error"
//! * The code must not use strings for error messages
//! * The production code must not use `unwrap` or `expect` (only tests may use `unwrap` or `expect`)
//! * If each field of each variant of the error enum implements `Copy`, then the error enum must implement `Copy` too
//! * Every fallible function body must begin with `use ThisFunctionError::*;`, where `ThisFunctionError` must be the name of this function's error enum
//! * The error handling code must use the error enum variant names without the error enum name prefix (for example: `ReadFileFailed` instead of `ParseConfigError::ReadFileFailed`)
//! * If the compiler emits a warning: "the `Err`-variant returned from this function is very large", then it's necessary to wrap some fields of the error in a `Box`
//!
//! ## Definitions
//!
//! ### Fallible expression
//!
//! An expression that returns a [`Result`].
//!
//! For example:
//!
//! ```rust
//! use std::collections::HashMap;
//! use error_handling::{handle, handle_bool, Display, Error};
//!
//! pub fn foo(numbers: Vec<u32>) -> Result<u32, FooError> {
//!     use FooError::*;
//!     // the following `if` is a fallible expression
//!     if numbers.is_empty() {
//!         return Err(NumbersAreEmpty { numbers });
//!     }
//!     // the following call to `find_even` is a fallible expression
//!     let result = find_even(numbers.clone().into_iter());
//!     let even = result.map_err(|source| FindEven { source })?;
//!     Ok(even)
//! }
//!
//! pub fn multiply_key(hashmap: HashMap<String, u32>, key: &str) -> Result<u32, MultiplyKeyError> {
//!     use MultiplyKeyError::*;
//!     // the following call chain if a fallible expression
//!     let value = hashmap.get(key).ok_or(KeyNotFound)?;
//!     Ok(*value * 10)
//! }
//!
//! pub fn find_even(mut numbers: impl Iterator<Item = u32>) -> Result<u32, FindEvenError> {
//!     use FindEvenError::*;
//!     numbers.find(|x| x % 2 == 0).ok_or(NotFound)
//! }
//!
//! #[derive(Error, Display, Debug)]
//! pub enum FooError {
//!     NumbersAreEmpty { numbers: Vec<u32> },
//!     FindEven { source: FindEvenError }
//! }
//!
//! #[derive(Error, Display, Debug)]
//! pub enum FindEvenError {
//!     NotFound
//! }
//!
//! #[derive(Error, Display, Debug)]
//! pub enum MultiplyKeyError {
//!     KeyNotFound
//! }
//! ```
//!
//! ### Data type
//!
//! A type that holds the actual data.
//!
//! For example:
//!
//! * `bool`
//! * `String`
//! * `PathBuf`
//!
//! ### Non-data type
//!
//! A type that doesn't hold the actual data.
//!
//! For example:
//!
//! * `RestClient` doesn't point to the actual data, it only allows querying it.
//! * `DatabaseConnection` doesn't hold the actual data, it only allows querying it.

pub use derive_more::Error;
pub use fmt_derive::Display;

mod macros;

mod types;

pub use types::*;
