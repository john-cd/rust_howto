<!-- TODO Reviewed ??-??-2026 -->

# Predicates

- [predicates][c~predicates~lib.rs]↗: Boolean-valued predicate functions.

A predicate is a function that returns `true` or `false`. In Rust, predicates are useful for building reusable, composable filters, validation checks, and runtime-configurable business logic.

This library implements an interface to "predicates" - boolean-valued functions of one argument. This allows combinatorial logic to be created and assembled at runtime and then used one or more times for evaluating values.

This is really useful when creating filters and checks that can be changed at runtime with user interaction - it allows a clean separation of concerns where the configuration code can be used to build up a predicate, and then that predicate can be given to the code that does the actual filtering without the filtering code knowing anything about user configuration.

```rust,editable
{{#include ../crates/cats/development_tools_testing/examples/predicates/main.rs:example}}
```

## Common Predicate Types

The `predicates` crate provides a wide range of built-in predicates:

- **String checks**: `contains`, `starts_with`, `ends_with`, `is_empty`, `matches` (regex).
- **Comparison**: `eq`, `ne`, `gt`, `ge`, `lt`, `le`.
- **Logic**: `and`, `or`, `not`, `always`, `never`.
- **Collections**: `in_iter`, `contains_ok`.
- **Path checks**: `exists`, `is_file`, `is_dir`.

## Why Use Predicates?

- Keep filtering and validation logic separate from the code that applies it.
- Build complex behavior from simple, reusable building blocks.
- Change filters dynamically at runtime without rewriting the core algorithm.

## Related Topics {#related-topics .skip}

- [[development-tools_testing | Testing]].

{{#include refs.incl.md}}
{{#include refs/link-refs.md}}
