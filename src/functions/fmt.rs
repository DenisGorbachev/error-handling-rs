use core::fmt::Formatter;

/// Formats the `error` using the [auto-deref trick](https://lukaskalbertodt.github.io/2019/12/05/generalized-autoref-based-specialization.html):
///
/// - If the `error` implements [`Errgonomic`](crate::Errgonomic), formats it using [`Errgonomic::fmt`](crate::Errgonomic::fmt)
/// - If the `error` implements [`Display`](std::fmt::Display), formats it using [`Display::fmt`](std::fmt::Display::fmt)
/// - Else returns a [formatting error](std::fmt::Error)
///
/// Note that if the `error` implements `Error`, then it also implements `Display` (it's a supertrait of `Error`).
pub fn fmt<E>(_error: E, _formatter: &mut Formatter<'_>) -> core::fmt::Result {
    todo!()
}
