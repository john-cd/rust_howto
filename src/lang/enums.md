# Enums

```rust,editable
{{#include ../../deps/tests/enums.rs}}
```

If we make an {{i:enum}} public, all of its {{i:variants}} are then public. We only need [`pub`][book-rust-reference-visibility-and-privacy]⮳ before the [`enum`][book-rust-reference-enum]⮳ keyword.

{{#include ../refs/link-refs.md}}
