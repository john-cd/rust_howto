# Match, if let, while let

```rust,editable
{{#include ../../deps/tests/match.rs}}
```

```rust,editable
{{#include ../../deps/tests/match2.rs}}
```

Patterns accept `1 | 2` for or, `1..=5` for inclusive range, `if x % 2 == 0` guards, @-binding `Message::Hello { id: id_variable @ 3..=7,}`.

```rust,editable
{{#include ../../deps/tests/if_let.rs}}
```

```rust,editable
{{#include ../../deps/tests/while_let.rs}}
```

## See Also

[Pattern matching][book-rust-pattern-matching]⮳

{{#include ../refs/link-refs.md}}
<div class="hidden">
TODO:
</div>
