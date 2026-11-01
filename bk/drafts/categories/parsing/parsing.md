<!-- TODO Reviewed ??-??-2026 -->

# Parsing Utilities

Use parser combinators to compose reusable parsing functions, or choose a parser generator when a separate grammar is easier to review and maintain. The examples below build custom parsers; for established formats such as JSON, TOML, Markdown, and XML, see [[parser-implementations | Parser Implementations]].

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

## Create a Parser with `tree-sitter` {#tree-sitter}

{{#badges tree-sitter}}

[`tree-sitter`][c~tree-sitter~docs]↗{{hi:tree-sitter}} provides Rust bindings to the Tree-sitter parsing library. [`tree-sitter`][tree-sitter~website]↗ is a parser generator tool and an incremental parsing library. It can build a concrete syntax tree for a source file and efficiently update the syntax tree as the source file is edited.

```rust,editable
{{#include ../../../crates/cats/parsing/examples/parsing/tree_sitter.rs:example}}
```

For editor-style token colors, see [[syntax_highlighting | Syntax Highlighting]].

## Create a Parser with `winnow` {#winnow}

{{#badges winnow parsing=}}

[`winnow`][c~winnow~docs]↗{{hi:winnow}} is a parser-combinator library derived from `nom`, with a focus on ergonomics and error reporting. A parser is a function that takes `&mut &str` and consumes the input it matches. The example combines `digit1` with `separated` to parse a comma-separated list of integers.

```rust,editable
{{#include ../../../crates/cats/parsing/examples/parsing/winnow.rs:example}}
```

## Highlight Source with `tree-sitter` Spans {#tree-sitter-spans}

{{#badges tree-sitter parsing=}}

Tree-sitter builds a concrete syntax tree that can be updated incrementally after an edit. The example walks the tree to collect the byte range of every node a highlighter would color, then edits the source and re-parses with the previous tree so only the changed region is re-analyzed. See also the basic `tree-sitter` example above.

```rust,editable
{{#include ../../../crates/cats/parsing/examples/parsing/tree_sitter_spans.rs:example}}
```

## Other Notable Crates {#other-notable-crates .skip}

- [`combine`][c~combine~docs]↗{{hi:combine}} and [`chumsky`][c~chumsky~docs]↗{{hi:chumsky}} are parser-combinator libraries; compare their error and recovery APIs when diagnostics are important.
- [`peg`][c~peg~docs]↗{{hi:peg}} generates a parser from a PEG grammar, like `pest`.

## Related Topics {#related-topics .skip}

- [[parser-implementations | Parser Implementations]].
- [[text-processing | Text Processing]].
- [[string_parsing | String Parsing]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
- [Awesome Pest][awesome-pest~repo]↗: A curated list of resources, projects, and tools using or for the `pest` parser generator in Rust.
- [ABNF to Pest][c~abnf_to_pest~crates.io]↗.
- [`winnow`][c~winnow~docs]↗.
- [parse-rosetta-rs][parse-rosetta-rs~repo]↗.

</div>
