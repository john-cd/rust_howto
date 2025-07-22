
# Operators

{{#include ops.incl.md}}

## Overload Operators like `+`, `-`, `[]` {#operator-overloading}

[![std][c~std~docs~badge]][c~std~docs]

Operator overloading allows you to define how operators like `+`, `-`, `*`, and `/` behave for your custom types. This is done by implementing the corresponding traits from the `std::ops` module.

Common overloaded operators include [`Add`](https://doc.rust-lang.org/std/ops/trait.Add.html)⮳, [`Mul`](https://doc.rust-lang.org/std/ops/trait.Mul.html)⮳ and [`Index`](https://doc.rust-lang.org/std/ops/trait.Index.html)⮳.

For example, to overload the `+` operator for a custom type, you would implement the `Add` trait:

```rust,editable
{{#include ../../crates/cats/standard_library/examples/ops/add.rs:example}}
```

## Related Topics {#related-topics}

- [[data_structures | Data Structures]].
- [[drop | Drop]].
- [[traits | Traits]].

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
</div>
