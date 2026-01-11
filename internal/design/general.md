# General design

* Some arguments that have been passed by value may already be unavailable when a specific fallible expression is executed
* Some values may have already been destructured when a specific fallible expression is executed
  * Examples
    * Market
      ```rust
      struct MarketRaw {
        slug: String,
        question_id: String,
      }
      
      struct Market {
        slug: String,
        question_id: QuestionId      
      }
      
      impl TryFrom<MarketRaw> for Market {
        type Error = ();
      
        fn try_from(market_raw: MarketRaw) -> Result<Self, Self::Error> {
          let MarketRaw {
            slug,
            question_id    
          } = market_raw;
          // if an error occurs while converting question_id from String to QuestionId, the `market` will already be unavailable
          // some other fields may have already been converted, too
          todo!()
        }
      }
      ```
* Some public crates export types that keep the relevant fields private, so they can only be accessed via `Debug` trait (for example: `xshell::Cmd` has a private `sh: Shell` field, which contains `cwd: PathBuf`, which is relevant to the call)
* Some public crates export types that have a `Debug` impl that doesn't explain the error (e.g. `toml_edit::Error` contains the whole TOML document and a span, so the user has to decipher the error by finding the relevant part of the document by the span)
* Some types don't implement `Display`, but every error enum must implement `Display` (e.g. `PathBuf`)
* `derive_more` and `fmt-derive` export derive macros which generate code which references these specific crates (so re-exporting the macros from this crate doesn't work out of the box)
  * Solutions
    * Write our own `Error` and `Display` macros
      * Implement a `Display` macro that defaults to "pretty" debug formatting
* Some errors must be handled in closures that are passed as arguments to `Iterator::map`
  * Such closures receive a reference to the current item, not an owned item, even if the outer function owns the iterator
  * Such closures should return an error that contains the information about the current item
    * Options
      * Index
        * This approach uses less memory and doesn't rely on the type implementing `Clone`, but requires more code
      * Clone
        * Properties
          * Requires the type to implement `Clone`
      * Ref
        * Conclusion: strictly worse than Clone
        * Pattern
          * Two errors: ClosureRefError<'a> and ClosureError, `impl<'a> From<ClosureRefError<'a>> for ClosureError`
        * Properties
          * Requires the type to implement `Clone`, because we can't extract the owned data from the iterator source just by reference (only if we compare the raw pointers, but that would require iterating over the data source the same numbers of times as there are errors)
      * Require using iterators that own the current item
        * Conclusion: can't work because some iterators will be created by external crates, so we can't enforce that `Iterator::Item` is owned
