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
//! * The name of the error type must end with `Error`
//! * The name of the error type must include the name of the function converted to CamelCase
//!   * If the function is a freestanding function, the name of the error type must be exactly equal to the name of the function converted to CamelCase concatenated with `Error`
//!   * If the function is an associated function, the name of the error type must be exactly equal to the name of the type without generics concatenated with the name of the function in CamelCase concatenated with `Error`
//!   * If the error is specified as an associated type of a foreign trait with multiple functions that return this associated error type, then the name of the error type must be exactly equal to the name of the trait including generics concatenated with the name of the type for which this trait is implemented concatenated with `Error`
//! * Every call to another fallible function must be wrapped in a unique error enum variant
//! * If the function contains only one fallible expression, this expression must still be wrapped in an error enum variant
//! * Every variable that contains secret data (the one which must not be displayed or logged, e.g. password, API key, personally identifying information) must have a type that doesn't output the underlying data in the Debug and Display impls (e.g. [`secrecy::SecretBox`](https://docs.rs/secrecy/latest/secrecy/struct.SecretBox.html))
//! * The code that calls a fallible function on each element of a collection should return an `impl Iterator<Item = Result<T, E>>` instead of short-circuiting on the first error
//! * If Clippy outputs a `result_large_err` warning, then the large fields of the error enum must be wrapped in a `Box`
//! * Every error enum variant must have a `#[display(...)]` attribute
//! * If the error enum variant has a `source` field, then the first argument of `#[display(...)]` attribute must end with "\n{source}"
//! * If the error type is defined for a `TryFrom<A> for B` impl, then its name must be equal to "Convert{A}To{B}Error"
//!
//! ## Notes
//!
//! * The name of the error enum should answer "what" failed, and its variants should answer "why" it failed
//! * Some arguments that have been passed by value may already be unavailable when a specific fallible expression is executed:
//! * Some public crates export types that keep the relevant fields private, so they can only be accessed via `Debug` trait (for example: `xshell::Cmd` has a private `sh: Shell` field, which contains `cwd: PathBuf`, which is relevant to the call)
//! * Some public crates export types that have a `Debug` impl that doesn't explain the error (e.g. `toml_edit::Error` contains the whole TOML document and a span, so the user has to decipher the error by finding the relevant part of the document by the span)
//! * Some types don't implement `Display`, but every error enum must implement `Display` (e.g. [`PathBuf`](std::path::PathBuf))
//! * `derive_more` and `fmt-derive` export derive macros which generate code which references these specific crates (so re-exporting the macros from this crate doesn't work out of the box)
//!   * Solutions
//!     * Write our own `Error` and `Display` macros
//!       * Implement a `Display` macro that defaults to "pretty" debug formatting
//!
//! ```rust
//! fn foo(a: String, b: String) {
//!     let a_new = bar(a);
//!     if b.is_empty() {
//!         // NOTE: `a` is unavailable here because it has been consumed by `bar`
//!         todo!()
//!     }  else {
//!         todo!()
//!     }
//! }
//!
//! fn bar(a: String) -> String {
//!     todo!()
//! }
//! ```
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
