# Fix writeln_error_to_writer to accept a generic `error: E` instead of `error: &(dyn Error + 'static)`

## Files

* src/functions/fmt.rs
* src/functions/writeln_error.rs
* src/types/err_vec.rs

## Tasks

* Implement `fn fmt`
* Fix writeln_error_to_writer to accept a generic `error: E`
  * Call `fmt`
* Fix ErrVec to accept a generic type parameter `T`
  * Add a comment: `T must implement Display or Errgonomic`
