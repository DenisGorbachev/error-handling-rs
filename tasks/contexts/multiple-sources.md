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
