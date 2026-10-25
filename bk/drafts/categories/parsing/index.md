<!-- TODO Reviewed ??-??-2026 -->

# Parsing

[![cat~parsing][cat~parsing~badge]][cat~parsing]{{hi:Parsing}}

Use this chapter when you are defining a grammar or parsing a custom text or binary format. It covers parser combinators, PEG grammars, parser generators, and lexers. For established formats such as JSON, TOML, Markdown, or XML, start with [[parser-implementations | Parser Implementations]], which compares format-specific libraries and runnable examples.

Choose a parser combinator such as [`nom`][c~nom~docs]↗{{hi:nom}}, [`winnow`][c~winnow~docs]↗{{hi:winnow}}, or [`chumsky`][c~chumsky~docs]↗{{hi:chumsky}} when you want to compose parsing logic in Rust. Choose [`pest`][c~pest~docs]↗{{hi:pest}} when a separate PEG grammar is easier to maintain, or [`lalrpop`][c~lalrpop~docs]↗{{hi:lalrpop}} for a generated LR parser. A lexer such as [`logos`][c~logos~docs]↗{{hi:logos}} is useful when tokenization is a distinct stage.

## Key Crates

| Topic                         | Rust Crates                                                                                                                                                                                           | Notes                                                                                                                                                                                                                                                                                                                                                                       |
| ----------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| General Parsing               | [`nom`][c~nom~docs]↗{{hi:nom}}, [`pest`][c~pest~docs]↗{{hi:pest}}, [`lalrpop`][c~lalrpop~docs]↗{{hi:lalrpop}}, [`chumsky`][c~chumsky~docs]↗{{hi:chumsky}}, [`combine`][c~combine~docs]↗{{hi:combine}} | [`nom`][c~nom~docs]↗{{hi:nom}} is good for binary formats and complex grammars. [`pest`][c~pest~docs]↗{{hi:pest}} is easy for text-based grammars. [`lalrpop`][c~lalrpop~docs]↗{{hi:lalrpop}} generates efficient parsers. [`chumsky`][c~chumsky~docs]↗{{hi:chumsky}} is a parser combinator library, [`combine`][c~combine~docs]↗{{hi:combine}} is another popular choice. |
| Lexing/Tokenization           | [`logos`][c~logos~docs]↗{{hi:logos}}, [`lexical`][c~lexical~docs]↗{{hi:lexical}}, [`regex`][c~regex~docs]↗{{hi:regex}}                                                                                | [`logos`][c~logos~docs]↗{{hi:logos}} generates fast lexers. [`lexical`][c~lexical~docs]↗{{hi:lexical}} is another lexer generator. [`regex`][c~regex~docs]↗{{hi:regex}} is useful for simpler tokenization or text processing.                                                                                                                                              |
| Parser Generators             | [`lalrpop`][c~lalrpop~docs]↗{{hi:lalrpop}}, [`pest`][c~pest~docs]↗{{hi:pest}}                                                                                                                         | These tools define the grammar and generate the parser code.                                                                                                                                                                                                                                                                                                                |
| Combinator Parsers            | [`nom`][c~nom~docs]↗{{hi:nom}}, [`chumsky`][c~chumsky~docs]↗{{hi:chumsky}}, [`combine`][c~combine~docs]↗{{hi:combine}}                                                                                | These libraries provide functions that can be combined to build more complex parsers.                                                                                                                                                                                                                                                                                       |
| Abstract Syntax Trees (AST)   | Often implemented in custom structs                                                                                                                                                                   | Parsers usually produce an AST, which represents the structure of the parsed input. You'll often define your own structs for this.                                                                                                                                                                                                                                          |
| Serialization/Deserialization | [`serde`][c~serde~docs]↗{{hi:serde}}                                                                                                                                                                  | To serialize or deserialize an AST, [`serde`][c~serde~docs]↗{{hi:serde}} is the standard choice.                                                                                                                                                                                                                                                                            |

## Code Examples

{{#include parsing.incl.md}}

## Chapter Guides

- [Parsing Utilities](parsing.md) introduces parser combinators, PEGs, and Tree-sitter.
- [Expression Parser](expression_parser.md) builds a lexer and precedence-aware parser for a small language.

## Related Topics

- [[argument_parsing | Argument Parsing]].
- [[parser-implementations | Parser Implementations]].
- [[text-processing | Text Processing]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
