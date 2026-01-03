use crate::Errgonomic;
use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result;

/// Formats the `error` using the [auto-deref trick](https://lukaskalbertodt.github.io/2019/12/05/generalized-autoref-based-specialization.html):
///
/// - If the `error` implements [`Errgonomic`], formats it using [`Errgonomic::fmt`]
/// - If the `error` implements [`Display`](Display), formats it using [`Display::fmt`](Display::fmt)
/// - Else returns a [formatting error](core::fmt::Error)
///
/// Note that if the `error` implements `Error`, then it also implements `Display` (it's a supertrait of `Error`).
#[allow(clippy::needless_borrow, clippy::needless_borrows_for_generic_args)]
pub fn fmt<E>(error: E, formatter: &mut Formatter<'_>) -> Result {
    let wrapped = AutoFmt(error);
    (&&&wrapped).fmt(formatter)
}

struct AutoFmt<T>(T);

impl<T: Errgonomic> Errgonomic for &&AutoFmt<T> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        Errgonomic::fmt(&self.0, formatter)
    }
}

// #[doc(hidden)]
// pub trait ViaDisplay {
//     fn fmt(&self, formatter: &mut Formatter<'_>) -> core::fmt::Result;
// }

impl<T: Display> Display for &AutoFmt<T> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        Display::fmt(&self.0, formatter)
    }
}

#[doc(hidden)]
pub trait Fallback {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result;
}

impl<T> Fallback for AutoFmt<T> {
    fn fmt(&self, _formatter: &mut Formatter<'_>) -> Result {
        Err(core::fmt::Error)
    }
}
