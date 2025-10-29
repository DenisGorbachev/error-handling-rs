use std::error::Error;
use std::fmt::{Display, Formatter};
use thiserror::Error;

/// This type is an attempt to implement error display for multiple errors through [`Display`] trait
#[derive(Error, Debug)]
pub struct ErrVecDisplay<T> {
    inner: Vec<T>,
}

impl<T: Error + 'static> Display for ErrVecDisplay<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("encountered {len} errors\n", len = self.inner.len()))?;
        for err in &self.inner {
            print_error(err, "  * ".to_string());
        }
        Ok(())
    }
}

pub fn print_error(error: &(dyn Error + 'static), prefix: String) {
    println!("{prefix}{error}");
    if let Some(source) = error.source() {
        print_error(source, prefix);
    }
}

#[derive(Error, Debug)]
pub enum FooError {
    #[error("bar failed")]
    BarFailed { source: BarError },
}

#[derive(Error, Debug)]
pub enum BarError {
    #[error("zeds failed")]
    ZedsFailed { source: ErrVecDisplay<ZedError> },
}

#[derive(Error, Debug)]
pub enum ZedError {
    #[error("ksa failed")]
    KsaFailed,
    #[error("pry failed")]
    PryFailed,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn must_err_vec_try() {
        let error = FooError::BarFailed {
            source: BarError::ZedsFailed {
                source: ErrVecDisplay {
                    inner: vec![ZedError::KsaFailed, ZedError::PryFailed],
                },
            },
        };
        print_error(&error, "- ".to_string());
    }
}
