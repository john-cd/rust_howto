# Structs

{{#include structs.incl.md}}

## Struct Syntax {#skip}

[![Rust by example - Structs][book-rust-by-example-structs-badge]][book-rust-by-example-structs]{{hi:Structs}}

Structs are custom data types that allow you to group related data together.

```rust,editable
{{#include ../../crates/language/tests/structs/structs.rs:example}}
```

Struct fields{{hi:Fields}} follow the general rule of everything being private by default{{hi:Private by default}} unless annotated with [`pub`][book-rust-reference-visibility-and-privacy]{{hi:Visibility}}⮳.

```rust,editable
{{#include ../../crates/language/tests/structs/structs2.rs:example}}
```

```rust,editable
{{#include ../../crates/language/tests/structs/structs3.rs:example}}
```

```rust,editable
{{#include ../../crates/language/tests/structs/structs4.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
[structs: add description NOW](https://github.com/john-cd/rust_howto/issues/559)
</div>
