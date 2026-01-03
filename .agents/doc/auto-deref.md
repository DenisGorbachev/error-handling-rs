# Auto-deref findings

These notes summarize the generalized autoref/auto-deref-based specialization approach from the article and how it applies to a three-tier fallback.

## Summary

- The pattern avoids overlapping impls by splitting each specialization level into its own trait.
- Autoref only adds at most one `&`, so it cannot scale beyond two levels; autoderef can be applied repeatedly, enabling arbitrarily many levels.
- Use auto-deref (not autoref) so you can stack many specialization levels; higher-priority levels use *more* references and the call site must include N references for N levels.
- A wrapper type (e.g., `AutoFmt<T>`) avoids interference from existing blanket impls on `&T` (like `Display for &T`).
- Each trait is implemented for a distinct reference depth (`&&AutoFmt<T>`, `&AutoFmt<T>`, `AutoFmt<T>`), so coherence is satisfied on stable Rust.
- Method resolution prefers candidates that require fewer auto-deref steps, which establishes the priority order.
- In generic functions, only impls whose bounds are satisfied by the function's `where` clauses are considered; specialization between *trait bounds* does not kick in unless those bounds are present.

## How the three-tier fallback works

1. Define three traits with the same method name (e.g., `fmt`):
   - `ViaErrgonomic` for `&&AutoFmt<T>` with bound `T: Errgonomic`.
   - `ViaDisplay` for `&AutoFmt<T>` with bound `T: Display`.
   - `ViaFallback` for `AutoFmt<T>` with no bounds.
2. Bring all three traits into scope and call the method on `&&&AutoFmt(value)` (the call must use *N references* where N is the number of specialization levels, matching the highest-priority impl's `self` type).
3. For a general N-level chain, the impl for level `i` uses `N - i - 1` references on `Wrap<...>`, and the call site uses `N` references on `Wrap(receiver)`.
4. Resolution behavior (method resolution considers `self` type, not `Self`):
   - If `T: Errgonomic`, the `&&AutoFmt<T>` impl matches with zero deref steps.
   - Else if `T: Display`, the `&AutoFmt<T>` impl matches after one deref step.
   - Otherwise, the fallback `AutoFmt<T>` impl matches after two deref steps and returns `fmt::Error`.

## Practical conclusion

The generalized auto-deref-based specialization technique supports a three-tier fallback (Errgonomic -> Display -> fmt::Error) on stable Rust, as long as the implementation uses distinct traits per tier and selects between them via reference depth on a wrapper type.
