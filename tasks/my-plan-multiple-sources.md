# Multiple sources

## Options

* Introduce `ErrVec` type
  * #plan
    * Modify `writeln_error`
      * Check if `error` can be downcast to `ErrVec`
        * If yes: get the sources from `ErrVec`
        * If no: get the sources from `Error::source`
    * Modify `handle_iter*` macros
      * Return an `ErrVec`
* Introduce `trait Errors` with `fn errors`
  * Blocked: can't be implemented for external types
