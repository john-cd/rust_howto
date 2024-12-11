# Match, if let, while let {#match}

{{#include match.incl.md}}

[![Rust by example - match][book-rust-by-example-match-badge]][book-rust-by-example-match]{{hi:match}}

```rust,editable
{{#include ../../deps/tests/language/match1.rs:example}}
```

```rust,editable
{{#include ../../deps/tests/language/match2.rs:example}}
```

Patterns accept `1 | 2` for or, `1..=5` for inclusive range, `if x % 2 == 0` guards, @-binding `Message::Hello { id: id_variable @ 3..=7,}`.

## `if let` {#if-let}

```rust,editable
{{#include ../../deps/tests/language/if_let.rs:example}}
```

## `while let` {#while-let}

```rust,editable
{{#include ../../deps/tests/language/while_let.rs:example}}
```

## See also

[Pattern matching][book-rust-pattern-matching]{{hi:Pattern matching}}⮳

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
TODO P1: add more / edit
</div>
