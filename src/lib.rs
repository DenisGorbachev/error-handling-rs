/*!
# Error handling

## Goal

The error handling code should help the caller to diagnose the issue, fix it, and retry the call.

## Approach

Every error must be represented by a unique enum variant with relevant data.

## Notes

Some arguments that have been passed by value may already be unavailable when a specific fallible expression is executed:

```rust
fn foo(a: String, b: String) {
    let a_new = bar(a);
    if b.is_empty() {
        // NOTE: `a` is unavailable here because it has been consumed by `bar`
        todo!()
    }  else {
        todo!()
    }
}

fn bar(a: String) -> String {
    todo!()
}
```

When calling a fallible function on each element of a collection, it is better to keep all Results instead of short-circuiting on the first error.

Wrap secret data in a type that doesn't output the secret data in the Debug and Display impls (e.g. [`secrecy::SecretBox`])
Keep the Copy types
Keep the relevant vars (passed by reference)
Note that some vars that were received by value may have already been moved into another function to produce another intermediate value; these can't be returned anymore - so we must always return an enum variant because different values will be available at different points
Handling Option types requires another macro (handle_opt!)
*/

mod types;

pub use types::*;
