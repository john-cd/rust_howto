<!-- TODO Reviewed ??-??-2026. -->

# Render a Diagnostic with a Source Span

When a lexer rejects a character, show where it failed and what input it
expected. [Logos](https://docs.rs/logos/) reports a byte range into the original
UTF-8 source. [codespan-reporting](https://docs.rs/codespan-reporting/) maps that
range to a displayed line and column, then draws a label under the error.

```rust,editable
{{#include ../../../crates/cats/compilers/examples/source_diagnostics.rs:example}}
```

Run from the repository root:

```console
just cargo run --manifest-path later/crates/Cargo.toml --locked -p compilers --example source_diagnostics
```

The report identifies `@` at column 8 even though `é` occupies two bytes. Preserve
byte ranges from the [lexer](tokenization.md) and pass the source text that
produced them. A parser can attach additional labels, for example to identify
the opening delimiter when a closing delimiter is missing.
