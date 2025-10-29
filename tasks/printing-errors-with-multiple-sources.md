# Implement printing errors with multiple sources

## Files

* src/lib.rs
* src/macros.rs
* src/functions/writeln_error.rs
* src/types/prefixer.rs

## Background

* Some error enum variants have a `sources` field, which contains a `Vec` of errors
* However, `writeln_error` prints a single chain of errors

## Tasks

* Modify `writeln_error` to print multiple chains of errors
  * Keep the same signature
  * Define `writeln_error_with_prefixer`
  * Use `Prefixer` if necessary (up to you)
* Un-ignore the tests (test functions marked with `#[ignore]`)
