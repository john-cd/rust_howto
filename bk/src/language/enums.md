# Enums {#enums}

{{#include enums.incl.md}}

[![Rust by example - Enums][book-rust-by-example-enums-badge]][book-rust-by-example-enums]{{hi:Enums}}

```rust,editable
{{#include ../../crates/language/tests/feat/enums.rs:example}}
```

If we make an enum{{hi:Enums}} public, all of its variants{{hi:Variants}} are then public. We only need [`pub`][book-rust-reference-visibility-and-privacy]{{hi:pub}}⮳ before the [`enum`][book-rust-reference-enum]⮳ keyword.

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
[enums: edit (P1)](https://github.com/john-cd/rust_howto/issues/542)

## See also

[[option | Option]]
[[result | Result]]

[[simple_data_types | Simple Data Types]]
[[structs | Structs]]

[[rust-patterns | Rust Patterns]]
[[match | Match]]

[[functional_programming | Functional Programming]]
</div>
