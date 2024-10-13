# Parsing

{{#include index.incl.md}}

## Parser Implementations

Parsers implemented for particular formats or languages.

[![cat-parser-implementations][cat-parser-implementations-badge]][cat-parser-implementations]

## Parsing utilities

Crates to help create parsers of binary and text formats. Format-specific parsers belong in other, more specific categories.

[![cat-parsing][cat-parsing-badge]][cat-parsing]

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
<div class="hidden">
TODO:

## Tree sitter

Tree-sitter is a parser generator tool and an incremental parsing library. It can build a concrete syntax tree for a source file and efficiently update the syntax tree as the source file is edited.

- [tree-sitter.github.io](https://tree-sitter.github.io/tree-sitter/)
- [tree-sitter (github)](https://github.com/tree-sitter/tree-sitter)

## Nom

- [nom](https://github.com/rust-bakery/nom/tree/main)
- [nominomicon](https://tfpk.github.io/nominomicon/introduction.html)

## Pest

- [pest.rs](https://pest.rs/)

pest is a general purpose parser written in Rust with a focus on **accessibility**, **correctness**, and **performance**. It uses [parsing expression grammars (or PEG)](https://en.wikipedia.org/wiki/Parsing_expression_grammar) as input, which are similar in spirit to regular expressions, but which offer the enhanced expressivity needed to parse complex languages.

Grammars are saved in separate `.pest` files which are never mixed with procedural code. This results in an always up-to-date formalization of a language that is easy to read and maintain.

## JS parser in Rust

Feature-complete es2019 parser: [swc_ecma_parser](https://crates.io/crates/swc_ecma_parser)

</div>
