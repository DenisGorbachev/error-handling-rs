# Naming design

## Requirements

* The names of the error enums must be unique within the crate
* The name of the error enum variant must be different from the name of the source error (to avoid name clashes)

## Notes

* The name of the error enum should answer "what happened", and its variants should answer "why it happened" (for example: `ParseConfigError::ReadFileFailed`)
* The name doesn't matter much (it is only displayed in the debug representation)
* Some naming patterns are commonplace:
  * `.*NotFound` (example: `UserNotFound`)
  * `Invalid.*` (example: `InvalidPassword`)
* It is natural to derive the name of the error from the message displayed to the user
  * Examples
    * Ex 1
      * "Passwords don't match" -> `PasswordsDoNotMatchError`
* Hypothesis: every error enum variant name may be constructed so that it ends with `Failed`
  * Examples
    * `UserNotFound` -> `FindUserFailed`
    * ``
  * Notes
    * This matches the pattern of naming the error types
      * Examples
        * `Users::find_by_name` -> `UsersFindByNameError` -> `enum FooError { UsersFindByNameFailed { source: UsersFindByNameError } }`
