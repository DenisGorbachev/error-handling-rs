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
    use derive_more::{Display, Error};
    use std::str::FromStr;

    #[test]
    fn must_handle_res() {
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
    }

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
