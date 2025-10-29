# Multiple sources

## Options

* Introduce `ErrVec` type
  * #plan
    * Introduce `ErrVec`
      * Must implement `Error`, because we must receive it from calling `Error::source`
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
