# Error handling guidelines

* Don't use `?` try operator - use the macros that begin with `handle`
* Use `handle!` to unwrap `Result` types
* Use `handle_opt!` to unwrap `Option` types
* Use `handle_bool!` to return an error if some condition is true
* Use `handle_iter!` or `handle_iter_of_refs!` to collect and return errors from iterators
* Note that macros that begin with `handle` already contain a `return` statement
* Don't call `.clone()` on the variables passed into error handling macros (there is no need to clone the variables because the macros consume them only in the error branch). The macros do not consume the variables that are passed into them in the success branch. If you call a macro, you can always use the variables that are passed into the macro call in the subsequent code as if they haven't been moved (because they actually are not moved in the success branch, only in the error branch).
* Use `thiserror` to derive `Error`
* Use `thiserror` version `2.0`
* Do not annotate any error enum variant fields with a `#[from]` attribute
* Do annotate every error enum variant with an `#[error]` attribute
  * The `#[error]` attribute must contain the error message displayed for the user
  * The `#[error]` attribute must not contain the `source` field
  * The `#[error]` attribute should contain only those fields that can be displayed on one line
  * If the `#[error]` attribute contains fields, then those fields must be wrapped in single quotes. This is necessary to correctly display fields that may contain spaces.
    * Good: `#[error("user '{name}' not found")]`
    * Bad: `#[error("user {name} not found")]`
