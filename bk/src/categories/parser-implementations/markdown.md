<!-- TODO Reviewed ??-??-2026. -->

# Markdown

{{#include markdown.incl.md}}

[`pulldown-cmark`][c~pulldown-cmark~docs]↗{{hi:pulldown-cmark}} implements CommonMark as an event stream. [`comrak`][c~comrak~docs]↗{{hi:comrak}} is a tree-based option with GitHub Flavored Markdown extensions.

## `pulldown-cmark` {#pulldown-cmark}

{{#badges pulldown-cmark text-processing="Text processing"}}{{hi:CommonMark}}{{hi:Markdown}}

[`pulldown-cmark`][c~pulldown-cmark~docs]↗{{hi:pulldown-cmark}} is a pull parser for CommonMark.

Use its event stream when processing Markdown without retaining a full syntax tree.

From `bk/crates`, check this example with `cargo test --locked -p parser_implementations --example markdown`.

<!-- Keep four backticks: the included Rust raw string contains triple-backtick fences. -->
<!-- dprint-ignore -->
````rust,noplayground
{{#include ../../../crates/cats/parser_implementations/examples/markdown/pulldown_cmark.rs:example}}
````

## `markdown` {#markdown}

{{#badges markdown compilers=Compilers encoding=Encoding parser-implementations="Parser implementations" parsing="Parsing tools" text-processing="Text processing"}}

{{hi:CommonMark}}
{{hi:markdown}}
{{hi:Parse}}
{{hi:Render}}
{{hi:Tokenize}}

[`markdown`][c~markdown~docs]↗{{hi:markdown}} is a CommonMark compliant markdown parser in Rust with ASTs and extensions.

Choose this crate when an owned abstract syntax tree is useful to later transformations.

Test this Cargo-backed example from `bk/crates` with `cargo test --locked -p parser_implementations --example markdown`.

```rust,editable,noplayground
{{#include ../../../crates/cats/parser_implementations/examples/markdown/markdown.rs:example}}
```

## `comrak` {#comrak}

{{#badges comrak command-line-utilities="Command line utilities" parsing="Parsing tools" text-processing="Text processing"}}{{hi:CommonMark}}{{hi:Markdown}}

[`comrak`][c~comrak~docs]↗{{hi:comrak}} is a 100% CommonMark-compatible GitHub Flavored Markdown parser and formatter.

Use `comrak` when GitHub Flavored Markdown extensions or rendering and formatting are part of the task.

```rust,editable,noplayground
{{#include ../../../crates/cats/parser_implementations/examples/markdown/comrak.rs:example}}
```

## Related Topics {#related-topics .skip}

- [Parsing Utilities](../parsing/parsing.md) explains how to construct parsers for custom grammars.
- [Text Processing](../text-processing/index.md) covers Markdown-adjacent string operations.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
</div>
