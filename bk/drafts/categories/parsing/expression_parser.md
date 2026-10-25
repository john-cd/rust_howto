<!-- TODO Reviewed ??-??-2026 -->

# Expression Parser

{{#include expression_parser.incl.md}}

## Write an Expression Parser with a `winnow` Lexer {#expression-parser}

[![winnow][c~winnow~docs~badge]][c~winnow~docs]{{hi:winnow}}
[![winnow~crates.io][c~winnow~crates.io~badge]][c~winnow~crates.io]
[![winnow~repo][c~winnow~repo~badge]][c~winnow~repo]
[![winnow~lib.rs][c~winnow~lib.rs~badge]][c~winnow~lib.rs]
[![cat~parsing][cat~parsing~badge]][cat~parsing]{{hi:Parsing tools}}

A small calculator shows the two stages found in most language front ends:

1. A **lexer** turns text into tokens. [`winnow`][c~winnow~docs]↗{{hi:winnow}} tokens can borrow from the input (zero-copy), and `LocatingSlice` plus `with_span` records the byte range of each token for error messages.
1. A **parser** turns tokens into a tree. Precedence climbing needs only a loop and a *binding power* per operator: a right-associative operator such as `^` gets a lower right power than left power. Unary minus is handled in the "atom" step.

The tree is then evaluated recursively. The tests check `1 + 2 * 3 = 7`, `2 ^ 3 ^ 2 = 512`, and the byte offsets reported in errors.

```rust,editable
{{#include ../../../crates/cats/parsing/examples/calc/calc_parser.rs:example}}
```

A hand-written parser is a good fit when the grammar is small, error messages matter, or the grammar is not context free in a convenient way. Prefer a grammar-driven tool such as [`pest`][c~pest~docs]↗{{hi:pest}} (see [[parsing | Parsing Utilities]]) when the grammar is large or changes often, and a combinator library such as [`nom`][c~nom~docs]↗{{hi:nom}} or `chumsky` for binary formats or when you want recovery from errors.

## Related Topics

- [[parsing | Parsing Utilities]].
- [[parser-implementations | Parser Implementations]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
