use std::error::Error;
use std::fmt::{Display, Formatter, Write as FmtWrite};
use thiserror::Error;

/// This type is an attempt to implement error display for multiple errors through [`Display`] trait
#[derive(Error, Debug)]
pub struct ErrVecDisplay<T> {
    inner: Vec<T>,
}

impl<T: Error + 'static> Display for ErrVecDisplay<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "encountered {len} errors", len = self.inner.len())?;
        self.inner.iter().try_for_each(|err| {
            f.write_str("\n")?;
            write_error_chain(f, err, "  * ")
        })
    }
}

pub fn format_error_chain(error: &(dyn Error + 'static), prefix: impl AsRef<str>) -> Result<String, std::fmt::Error> {
    let mut output = String::new();
    write_error_chain(&mut output, error, prefix.as_ref())?;
    Ok(output)
}

fn write_error_chain(writer: &mut impl FmtWrite, error: &(dyn Error + 'static), prefix: &str) -> std::fmt::Result {
    write!(writer, "{prefix}{error}")?;
    if let Some(source) = error.source() {
        writer.write_str("\n")?;
        write_error_chain(writer, source, prefix)
    } else {
        Ok(())
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
        use BarError::*;
        use FooError::*;
        use ZedError::*;

        let error = BarFailed {
            source: ZedsFailed {
                source: ErrVecDisplay {
                    inner: vec![KsaFailed, PryFailed],
                },
            },
        };
        let output = format_error_chain(&error, "- ").unwrap();
        assert_eq!(output, include_str!("err_vec_display/fixtures/must_err_vec_display.txt"));
    }
}
