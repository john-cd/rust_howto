# Match, if let, While let

{{#include match.incl.md}}

## Match {#match}

[![Rust by example - match][book-rust-by-example-match-badge]][book-rust-by-example-match]{{hi:match}}

## Use `match` to Branch on a Pattern {#use-match-to-branch-on-a-pattern}

The following code demonstrates pattern matching against an enumeration (enum):

```rust,editable
{{#include ../../crates/language/tests/match/match1.rs:example}}
```

This examples shows pattern matching using a `struct`:

```rust,editable
{{#include ../../crates/language/tests/match/match2.rs:example}}
```

Patterns accept `1 | 2` for or, `1..=5` for inclusive range, `if x % 2 == 0` guards, @-binding `Message::Hello { id: id_variable @ 3..=7,}`.

## `if let` {#if-let}

[![Rust by example - if let][book-rust-by-example-if_let-badge]][book-rust-by-example-if_let]

`if let` is a concise way to handle a single pattern in a `match` expression.

```rust,editable
{{#include ../../crates/language/tests/match/if_let.rs:example}}
```

## `while let` {#while-let}

`while let` is similar to `if let`, but it allows you to loop as long as the pattern continues to match.

```rust,editable
{{#include ../../crates/language/tests/match/while_let.rs:example}}
```

## See Also {#skip}

- [[control_flow | Control Flow]].
- [[error_handling | Error Handling]].
- [[option | Option]].
- [[result | Result]].
- [[rust-patterns | Rust Patterns]].

## References {#skip}

- [Pattern matching][book-rust-pattern-matching]{{hi:Pattern matching}}⮳.

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
[match: add more / edit NOW](https://github.com/john-cd/rust_howto/issues/551)
</div>
