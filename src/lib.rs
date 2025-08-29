/*!
# Error handling

## Goal

The error handling code should help the caller to diagnose the issue, fix it, and retry the call.

## Approach

Every error must be represented by a unique enum variant with relevant data.

## Notes

Some arguments that have been passed by value may already be unavailable when a specific fallible expression is executed:

```rust
fn foo(a: String, b: String) {
    let a_new = bar(a);
    if b.is_empty() {
        // NOTE: `a` is unavailable here because it has been consumed by `bar`
        todo!()
    }  else {
        todo!()
    }
}

fn bar(a: String) -> String {
    todo!()
}
```

When calling a fallible function on each element of a collection, it is better to keep all Results instead of short-circuiting on the first error.

Wrap secret data in a type that doesn't output the secret data in the Debug and Display impls (e.g. [`secrecy::SecretBox`])
Keep the Copy types
Keep the relevant vars (passed by reference)
Note that some vars that were received by value may have already been moved into another function to produce another intermediate value; these can't be returned anymore - so we must always return an enum variant because different values will be available at different points
Handling Option types requires another macro (handle_opt!)

## Definitions

### Fallible expression

An expression that returns a [`Result`].

For example:

```rust
use error_handling::{handle, handle_bool};
use derive_more::{Error, Display};

pub fn foo(numbers: Vec<u32>) -> Result<u32, FooError> {
    use FooError::*;
    // the following `if` is a fallible expression
    if numbers.is_empty() {
        return Err(NumbersAreEmpty { numbers });
    }
    // the following call to `find_even` is a fallible expression
    let result = find_even(numbers.clone().into_iter());
    let even = result.map_err(|source| FindEven { source })?;
    Ok(even)
}

pub fn find_even(mut numbers: impl Iterator<Item = u32>) -> Result<u32, FindEvenError> {
    use FindEvenError::*;
    numbers.find(|x| x % 2 == 0).ok_or(NotFound)
}

#[derive(Error, Display, Debug)]
pub enum FooError {
    #[display("Numbers are empty: {numbers:#?}")]
    NumbersAreEmpty { numbers: Vec<u32> },
    FindEven { source: FindEvenError }
}

#[derive(Error, Display, Debug)]
pub enum FindEvenError {
    NotFound
}
```

*/

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
    ($var:ident, $option:expr, $variant:ident$(,)? $($arg:ident$(: $value:expr)?),*) => {
        let Some($var) = $option else {
            return Err($variant {
                $($arg$(: $value)?),*
            });
        };
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
