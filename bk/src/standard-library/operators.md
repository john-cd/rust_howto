
# Operators

{{#include operators.incl.md}}

## Overload Operators like `+`, `-`, `[]` {#operator-overloading}

[![std][c~std~docs~badge]][c~std~docs]

Operator overloading allows you to define how operators like `+`, `-`, `*`, and `/` behave for your custom types. This is done by implementing the corresponding traits from the `std::ops` module.

Common overloaded operators include [`Add`][c~std::ops::Add~docs]↗, [`Mul`][c~std::ops::Mul~docs]↗ and [`Index`][c~std::ops::Index~docs]↗.

For example, to overload the `+` operator for a custom type, you would implement the `Add` trait:

```rust,editable
{{#include ../../crates/standard_library/examples/ops/ops.rs:example}}
```

## Add Function-like Behavior to a Custom Type {#function-like-behavior}

The `std::ops` module also provides traits for function-like / closure-like behavior: `Fn`, `FnMut`, and `FnOnce`. You can make your own types callable like functions by implementing these traits. This is _unstable_ and requires nightly Rust:

```rust,editable
{{#include ../../crates/standard_library/examples/ops/callable.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[data_structures | Data Structures]].
- [[drop | `Drop`]].
- [[traits | Traits]].

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
</div>
