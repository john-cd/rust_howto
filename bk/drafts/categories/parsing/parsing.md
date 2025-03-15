# Parsing Utilities

{{#include parsing.incl.md}}

## Create a Parser with `nom` {#nom}

[![nom][c-nom-badge]][c-nom]{{hi:nom}}
[![nom-crates.io][c-nom-crates.io-badge]][c-nom-crates.io]
[![nom-github][c-nom-github-badge]][c-nom-github]
[![nom-lib.rs][c-nom-lib.rs-badge]][c-nom-lib.rs]
[![cat-parsing][cat-parsing-badge]][cat-parsing]{{hi:Parsing tools}}

[nom][c-nom-github]{{hi:nom}}⮳ is a byte-oriented, zero-copy, parser combinators library.

See also: the [nominomicon][c-nom-nominomicon]⮳ book.

```rust,editable
{{#include ../../../crates/cats/parsing/tests/nom.rs:example}}
```

## Create a Parser with `pest` {#pest}

[![pest][c-pest-badge]][c-pest]{{hi:pest}}
[![pest-crates.io][c-pest-crates.io-badge]][c-pest-crates.io]
[![pest-github][c-pest-github-badge]][c-pest-github]
[![pest-lib.rs][c-pest-lib.rs-badge]][c-pest-lib.rs]
[![cat-parsing][cat-parsing-badge]][cat-parsing]{{hi:Parsing tools}}

[`pest`][c-pest-website]{{hi:pest}}⮳ is a general-purpose parser written in Rust, with a focus on accessibility, correctness, and performance. It uses [parsing expression grammars (or PEG)][wikipedia-parsing-expression-grammars]⮳ as input, which are similar in spirit to regular expressions, but which offer the enhanced expressivity needed to parse complex languages.

Grammars are saved in separate [`.pest`][c-pest]⮳{{hi:.pest}}  files which are never mixed with procedural code. This results in an always up-to-date formalization of a language that is easy to read and maintain.

```rust,editable
{{#include ../../../crates/cats/parsing/tests/pest/pest.rs:example}}
```

### Create a Parser with `tree-sitter` {#tree-sitter}

[![tree-sitter][c-tree_sitter-badge]][c-tree_sitter]{{hi:tree-sitter}}
[![tree-sitter-crates.io][c-tree_sitter-crates.io-badge]][c-tree_sitter-crates.io]
[![tree-sitter-github][c-tree_sitter-github-badge]][c-tree_sitter-github]
[![tree-sitter-lib.rs][c-tree_sitter-lib.rs-badge]][c-tree_sitter-lib.rs]

[`tree-sitter`][c-tree_sitter]⮳{{hi:tree-sitter}} provides Rust bindings to the Tree-sitter parsing library. [`tree-sitter`][tree-sitter-website]⮳ is a parser generator tool and an incremental parsing library. It can build a concrete syntax tree for a source file and efficiently update the syntax tree as the source file is edited.

```rust,editable
{{#include ../../../crates/cats/parsing/tests/tree_sitter.rs:example}}
```

## Other Notable Crates {#skip}

- [`combine`][c-combine]⮳{{hi:combine}}.
- `peg`.
- [`chumsky`][c-chumsky]⮳{{hi:chumsky}}.

## Related Topics {#skip}

- [[parser-implementations | Parser Implementations]].
- [[text-processing | Text Processing]].
- [[string_parsing | String Parsing]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[parsing: write; examples](https://github.com/john-cd/rust_howto/issues/448)
</div>
