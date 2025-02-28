# Parsing

[![cat-parsing][cat-parsing-badge]][cat-parsing]{{hi:Parsing}}

This chapter focuses on writing parsers for binary and text formats. Existing format-specific parsers belong in other, more specific categories, for example within [[parser-implementations | Parser Implementations]].

## Key Crates

| Topic | Rust Crates | Notes |
| --- | --- | --- |
| General Parsing | [`nom`][c-nom]⮳{{hi:nom}}, [`pest`][c-pest]⮳{{hi:pest}}, [`lalrpop`][c-lalrpop]⮳{{hi:lalrpop}}, [`chumsky`][c-chumsky]⮳{{hi:chumsky}}, [`combine`][c-combine]⮳{{hi:combine}} | [`nom`][c-nom]⮳{{hi:nom}} is good for binary formats and complex grammars. [`pest`][c-pest]⮳{{hi:pest}} is easy for text-based grammars. [`lalrpop`][c-lalrpop]⮳{{hi:lalrpop}} generates efficient parsers. [`chumsky`][c-chumsky]⮳{{hi:chumsky}} is a parser combinator library, [`combine`][c-combine]⮳{{hi:combine}} is another popular choice. |
| Lexing/Tokenization | [`logos`][c-logos]⮳{{hi:logos}}, [`lexical`][c-lexical]⮳{{hi:lexical}}, [`regex`][c-regex]⮳{{hi:regex}} | [`logos`][c-logos]⮳{{hi:logos}} generates fast lexers. [`lexical`][c-lexical]⮳{{hi:lexical}} is another lexer generator. [`regex`][c-regex]⮳{{hi:regex}} is useful for simpler tokenization or text processing. |
| Parser Generators | [`lalrpop`][c-lalrpop]⮳{{hi:lalrpop}}, [`pest`][c-pest]⮳{{hi:pest}} | These tools define the grammar and generate the parser code. |
| Combinator Parsers | [`nom`][c-nom]⮳{{hi:nom}}, [`chumsky`][c-chumsky]⮳{{hi:chumsky}}, [`combine`][c-combine]⮳{{hi:combine}} | These libraries provide functions that can be combined to build more complex parsers. |
| Abstract Syntax Trees (AST) | Often implemented in custom structs | Parsers usually produce an AST, which represents the structure of the parsed input. You'll often define your own structs for this. |
| Serialization/Deserialization | [`serde`][c-serde]⮳{{hi:serde}} | If you need to serialize or deserialize an AST, [`serde`][c-serde]⮳{{hi:serde}} is the standard choice. |

## Code Examples

{{#include parsing.incl.md}}

## Related Topics

- [[argument_parsing | Argument Parsing]]
- [[parser-implementations | Parser Implementations]]
- [[text-processing | Text Processing]]

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[P2 review](https://github.com/john-cd/rust_howto/issues/952)

</div>
