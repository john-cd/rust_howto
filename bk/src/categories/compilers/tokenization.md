<!-- TODO Reviewed ??-??-2026. -->

# Tokenizing Input with Logos

A lexer turns source text into a sequence of tokens that a parser can consume. [logos][c~logos~docs]↗{{hi:logos}} derives an efficient lexer from token definitions and regular expressions. Use it when token boundaries can be described by regular patterns; a lexer does not check expression precedence, matching delimiters, or the grammar as a whole.

The example recognizes identifiers and an addition operator, skips spaces, and asserts the token sequence. A real lexer should retain each token's source span so parser errors can point to the original text. Logos yields an error for an unrecognized token. Use its byte range to [render a source diagnostic](source_diagnostics.md). Run this example from `bk/crates` with `cargo run --locked -p compilers --example tokenization`.

```rust,noplayground
{{#include ../../../crates/cats/compilers/examples/tokenization/tokenization.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[parsing | Parsing]] and [[parser-implementations | Parser Implementations]].
- [[compilers | Compilers]].

{{#include refs.incl.md}}
