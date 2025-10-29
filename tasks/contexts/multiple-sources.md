# Implement printing errors with multiple sources

## Files

* src/lib.rs
* src/macros.rs
* src/functions/writeln_error.rs
* src/types/prefixer.rs

## Background

* Some error enum variants have a `sources` field, which contains a `Vec` of errors
* However, `writeln_error` prints a single chain of errors
* This happens because the `Error` trait has an `Error::source` method which returns only a single error

## Requirements

* Must support errors from external crates
  * This requirement rules out external traits, because the users of this crate won't be able to implement an external trait for an external type
* Must not require `nightly` features

## Alternatives

* Introduce `ErrVec` type
  * #alternatives
    * Introduce `pub struct ErrVec { pub inner: Vec<Box<dyn Error + 'static>> }`
      * #notes
        * This approach is currently implemented
        * This approach hides the underlying error type behind `dyn Error`
          * But the user may downcast to a specific error type (the user knows the specific error type because the user knows what specific error enum variant he is matching on)
    * Introduce `pub struct ErrVec<E: Error + 'static> { pub inner: Vec<E> }`
      * #notes
        * This approach is blocked on a problem: how to downcast an `error: &(dyn Error + 'static)` to a `ErrVec<E>`? (the generic `E` is problematic)
      * #alternatives
        * Use a custom `impl<E> Display for ErrVec<E>` to display all underlying errors
  * #requirements
    * Must implement `Error`, because we must receive it from `Error::source` call
  * #plan
    * Introduce `ErrVec`
    * Modify `writeln_error`
      * Check if `error` can be downcast to `ErrVec`
        * If yes: get the sources from `ErrVec`
        * If no: get the sources from `Error::source`
    * Modify `handle_iter*` macros
      * Return an `ErrVec`
    * Modify the userland code
      * Modify the error enum variants with `sources` field
        * Rename the field names from `sources` to `source`
        * Change the type from `Vec` to `ErrVec`
* Introduce `trait Errors` with `fn errors`
  * Blocked: can't be implemented for external types
