<!-- TODO Reviewed ??-??-2026. -->

# Parsing Utilities

Use parser combinators to compose reusable parsing functions, or choose a parser generator when a separate grammar is easier to review and maintain. The examples below build custom parsers; for established formats such as JSON, TOML, Markdown, and XML, prefer a format-specific library.

{{#include parsing.incl.md}}

## Create a Parser with `nom` {#nom}

{{#badges nom parsing="Parsing tools"}}

[nom][c~nom~repo]↗{{hi:nom}} is a byte-oriented, zero-copy, parser combinators library.

See also: the [nominomicon][c~nom~nominomicon]↗ book.

```rust,editable
{{#include ../../../crates/cats/parsing/examples/parsing/nom.rs:example}}
```

## Create a Parser with `pest` {#pest}

{{#badges pest parsing="Parsing tools"}}

[`pest`][c~pest~website]↗{{hi:pest}} is a general-purpose parser written in Rust, with a focus on accessibility, correctness, and performance. It uses [parsing expression grammars (or PEG)][wikipedia~parsing-expression-grammars]↗ as input, which are similar in spirit to regular expressions, but which offer the enhanced expressivity needed to parse complex languages.

Grammars are saved in separate [`.pest`][c~pest~docs]↗{{hi:.pest}}  files which are never mixed with procedural code. This results in an always up-to-date formalization of a language that is easy to read and maintain.

```rust,editable
{{#include ../../../crates/cats/parsing/examples/pest/pest.rs:example}}
```

### Find Markdown Link Candidates

The document grammar consumes fenced code blocks before looking for inline links. The example walks the parse tree and prints the source span for each inline link candidate. Reference labels and nested links require additional context, and applications that need complete CommonMark interpretation should use a Markdown parser.

```rust,noplayground
{{#include ../../../crates/cats/parsing/examples/pest_markdown/main.rs:example}}
```

See the [full Pest grammar](https://github.com/john-cd/rust_howto/blob/develop/bk/crates/cats/parsing/examples/pest_markdown/markdown_links.pest). Run the example from `bk/crates` with `cargo run -p parsing --example pest_markdown`.

## Create a Parser with `tree-sitter` {#tree-sitter}

{{#badges tree-sitter}}

[`tree-sitter`][c~tree-sitter~docs]↗{{hi:tree-sitter}} provides Rust bindings to the Tree-sitter parsing library. [`tree-sitter`][tree-sitter~website]↗ is a parser generator tool and an incremental parsing library. It can build a concrete syntax tree for a source file and efficiently update the syntax tree as the source file is edited.

From `bk/crates`, run `cargo test --locked -p parsing --example parsing`. The tests parse source and check query matches.

```rust,ignore,noplayground
{{#include ../../../crates/cats/parsing/examples/parsing/tree_sitter.rs:example}}
```

The byte spans from the syntax-tree examples can also drive editor-style token coloring.

## Create a Parser with `winnow` {#winnow}

{{#badges winnow parsing=}}

[`winnow`][c~winnow~docs]↗{{hi:winnow}} is a parser-combinator library derived from `nom`, with a focus on ergonomics and error reporting. A parser is a function that takes `&mut &str` and consumes the input it matches. The example combines `digit1` with `separated` to parse a comma-separated list of integers.

```rust,editable
{{#include ../../../crates/cats/parsing/examples/parsing/winnow.rs:example}}
```

## Highlight Source with `tree-sitter` Spans {#tree-sitter-spans}

{{#badges tree-sitter parsing=}}

Tree-sitter builds a concrete syntax tree that can be updated incrementally after an edit. The example walks the tree to collect the byte range of every node a highlighter would color, then edits the source and re-parses with the previous tree so only the changed region is re-analyzed. See also the basic `tree-sitter` example above.

From `bk/crates`, run `cargo test --locked -p parsing --example tree_sitter_spans`. The tests check syntax spans before and after an edit.

```rust,ignore,noplayground
{{#include ../../../crates/cats/parsing/examples/parsing/tree_sitter_spans.rs:example}}
```

## Other Notable Crates {#other-notable-crates .skip}

- [`combine`][c~combine~docs]↗{{hi:combine}} and [`chumsky`][c~chumsky~docs]↗{{hi:chumsky}} are parser-combinator libraries; compare their error and recovery APIs when diagnostics are important.
- [`peg`][c~peg~docs]↗{{hi:peg}} generates a parser from a PEG grammar, like `pest`.

## Further Reading

- [Awesome Pest][awesome-pest~repo]↗ collects projects and learning resources for Pest.
- [Parse Rosetta Rust][parse-rosetta-rs~repo]↗ compares parser approaches on a shared task.

## Related Topics {#related-topics .skip}

- [Parsing](index.md).
- [Strings](../data-structures/strings.md).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
