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
//! * The name of the error type must end with `Error`
//! * The name of the error type must include the name of the function converted to CamelCase
//!   * If the function is a freestanding function, the name of the error type must be exactly equal to the name of the function converted to CamelCase concatenated with `Error`
//!   * If the function is an associated function, the name of the error type must be exactly equal to the name of the type without generics concatenated with the name of the function in CamelCase concatenated with `Error`
//!   * If the error is specified as an associated type of a foreign trait with multiple functions that return this associated error type, then the name of the error type must be exactly equal to the name of the trait including generics concatenated with the name of the type for which this trait is implemented concatenated with `Error`
//! * Every call to another fallible function must be wrapped in a unique error enum variant
//! * If the function contains only one fallible expression, this expression must still be wrapped in an error enum variant
//! * Every variable that contains secret data (the one which must not be displayed or logged, e.g. password, API key, personally identifying information) must have a type that doesn't output the underlying data in the Debug and Display impls (e.g. [`secrecy::SecretBox`](https://docs.rs/secrecy/latest/secrecy/struct.SecretBox.html))
//! * The code that calls a fallible function on each element of a collection should return an `impl Iterator<Item = Result<T, E>>` instead of short-circuiting on the first error
//!
//! ## Notes
//!
//! * The name of the error enum should answer "what" failed, and its variants should answer "why" it failed
//! * Some arguments that have been passed by value may already be unavailable when a specific fallible expression is executed:
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
//! use error_handling::{handle, handle_bool};
//! use derive_more::{Error, Display};
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
//!     #[display("Numbers are empty: {numbers:#?}")]
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
//!

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
                source,
                $($arg$(: $value)?),*
            }),
        }
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
                $($arg$(: $value)?),*
            }),
        }
    };
}

#[macro_export]
macro_rules! handle_bool {
    ($condition:expr, $variant:ident$(,)? $($arg:ident$(: $value:expr)?),*) => {
        if $condition {
            return Err($variant {
                $($arg$(: $value)?),*
            });
        };
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use derive_more::{Display, Error};

    #[test]
    fn must_handle_opt() {
        #[allow(dead_code)]
        fn find_even(numbers: Vec<u32>) -> Result<u32, FindEvenError> {
            use FindEvenError::*;
            let even = handle_opt!(numbers.iter().find(|x| *x % 2 == 0), NotFound);
            Ok(*even)
        }
        #[derive(Error, Display, Debug)]
        enum FindEvenError {
            NotFound,
        }
    }
}
