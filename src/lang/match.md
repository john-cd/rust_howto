# Match, if let, while let

[![Rust by example - match][book-rust-by-example-match-badge]][book-rust-by-example-match]{{hi:match}}

```rust
{{#include ../../deps/tests/lang/match1.rs:example}}
```

```rust
{{#include ../../deps/tests/lang/match2.rs:example}}
```

Patterns accept `1 | 2` for or, `1..=5` for inclusive range, `if x % 2 == 0` guards, @-binding `Message::Hello { id: id_variable @ 3..=7,}`.

```rust
{{#include ../../deps/tests/lang/if_let.rs:example}}
```

```rust
{{#include ../../deps/tests/lang/while_let.rs:example}}
```

## See Also

[Pattern matching][book-rust-pattern-matching]{{hi:Pattern matching}}⮳

{{#include ../refs/link-refs.md}}

<div class="hidden">
TODO: add more / edit
</div>
