# Match, if let, while let

```rust,editable
{{#include ../../deps/examples/match.rs}}
```

```rust,editable
{{#include ../../deps/examples/match2.rs}}
```

Patterns accept `1 | 2` for or, `1..=5` for inclusive range, `if x % 2 == 0` guards, @-binding `Message::Hello { id: id_variable @ 3..=7,}`.

```rust,editable
{{#include ../../deps/examples/if_let.rs}}
```

```rust,editable
{{#include ../../deps/examples/while_let.rs}}
```

## See Also

[Pattern matching][pattern-matching]⮳

{{#include ../link-refs.md}}
