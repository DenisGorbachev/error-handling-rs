# Rust Crate Documentation Instructions

## General

- Edit files in `src` only
- Don't edit `README.md` (it's auto-generated)

## Crate-Level Documentation (lib.rs)

- Start with a one-paragraph summary that explains what the crate does.
- Provide a minimal quickstart example that compiles on stable Rust.
- Provide a second example that demonstrates a non-trivial, realistic use case.
- Explain the core concepts and mental model in plain language.
- List the crate's main types and how they relate.
- If the crate has feature flags: add a "Feature flags" section with a short description for each feature.
- If the crate supports only specific targets: document those targets.
- If the crate has system dependencies: document those dependencies.
- If the crate relies on a global system-wide configuration: document this configuration.

## Item Documentation (Types, Traits, Functions, Macros, Modules)

- Document every public item.
- Use a one-line summary followed by details only when needed.
- Do not add an `# Arguments` section.
- Do not describe argument lists; demonstrate usage through examples instead.
- Document invariants, if present.
- Document side effects, if present.
- Document panic behavior, if present.
- Document safety invariants for `unsafe` functions.
- Document trait contracts, if present.

## Doctests

- Use `#`-prefixed hidden lines for setup or imports.
- Ensure examples compile against the public API without private items.
- Prefer deterministic examples; avoid external network or filesystem dependencies.

## Links, Formatting, and Style

- Use intra-doc links like [`Type`], [`Trait`].
- Use Rust code fences (` ```rust `) and ensure they compile.
