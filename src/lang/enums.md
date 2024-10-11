# Enums

```rust,editable
{{#include ../../deps/tests/enums.rs}}
```

If we make an enum{{hi:enum}} public, all of its variants{{hi:variants}} are then public. We only need [`pub`][book-rust-reference-visibility-and-privacy]{{hi:pub}}⮳ before the [`enum`][book-rust-reference-enum]{{hi:enum}}⮳ keyword.

{{#include ../refs/link-refs.md}}
