<!-- TODO Reviewed ??-??-2026 -->

# Expression Parser

{{#include expression_parser.incl.md}}

## Write an Expression Parser with a `winnow` Lexer {#expression-parser}

{{#badges winnow parsing="Parsing tools"}}

A small calculator shows the two stages found in most language front ends:

1. A **lexer** turns text into tokens. [`winnow`][c~winnow~docs]↗{{hi:winnow}} tokens can borrow from the input (zero-copy), and `LocatingSlice` plus `with_span` records the byte range of each token for error messages.
1. A **parser** turns tokens into a tree. Precedence climbing needs only a loop and a _binding power_ per operator: a right-associative operator such as `^` gets a lower right power than left power. Unary minus is handled in the "atom" step.

The tree is then evaluated recursively. The tests check `1 + 2 * 3 = 7`, `2 ^ 3 ^ 2 = 512`, and the byte offsets reported in errors.

```rust,editable
{{#include ../../../crates/cats/parsing/examples/calc/calc_parser.rs:example}}
```

A hand-written parser is a good fit when the grammar is small, error messages matter, or the grammar is not context free in a convenient way. Prefer a grammar-driven tool such as [`pest`][c~pest~docs]↗{{hi:pest}} (see [[parsing | Parsing Utilities]]) when the grammar is large or changes often, and a combinator library such as [`nom`][c~nom~docs]↗{{hi:nom}} or `chumsky` for binary formats or when you want recovery from errors.

## Hand-Written Parser, `pest`, `chumsky` or `nom` {#parser-choices}

[![pest][c~pest~docs~badge]][c~pest~docs] [![chumsky][c~chumsky~docs~badge]][c~chumsky~docs] [![nom][c~nom~docs~badge]][c~nom~docs] [![winnow][c~winnow~docs~badge]][c~winnow~docs]{{hi:pest}}{{hi:chumsky}}{{hi:nom}}{{hi:winnow}} [![cat~parsing][cat~parsing~badge]][cat~parsing]{{hi:Parsing tools}}

The calculator above is hand-written on top of a lexer and a precedence-climbing loop. That is a good choice for a small, stable grammar: no extra build step, full control over error messages and recovery, and few dependencies. Consider a library when the grammar is large or changes often:

- [`pest`][c~pest~docs]↗{{hi:pest}} generates a parser from a PEG grammar file. The grammar is readable and checked at compile time, but you get a generic parse tree to convert into your own types, and error messages are generic unless you customize them.
- [`chumsky`][c~chumsky~docs]↗{{hi:chumsky}} is a parser-combinator library focused on good error reporting and error recovery, which suits programming languages and editors.
- [`nom`][c~nom~docs]↗{{hi:nom}} is a fast parser-combinator library for byte and text formats; it is a frequent choice for binary protocols and streaming input. [`winnow`][c~winnow~docs]↗{{hi:winnow}}, used above, is a nom-derived library with a friendlier API and error handling.

A rule of thumb: a hand-written parser for one small language, `pest` when a separate grammar file helps readers, `chumsky` when diagnostics matter, `nom` or `winnow` for data formats. Whichever you pick, keep the lexer or tokenizer tests and the byte spans: they are what make error messages useful.

## Related Topics

- [[parsing | Parsing Utilities]].
- [[parser-implementations | Parser Implementations]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
