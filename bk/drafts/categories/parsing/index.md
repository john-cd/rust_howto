# Parsing

[![cat~parsing][cat~parsing~badge]][cat~parsing]{{hi:Parsing}}

This chapter focuses on writing parsers for binary and text formats. Existing format-specific parsers belong in other, more specific categories, for example within [[parser-implementations | Parser Implementations]].

## Key Crates

| Topic | Rust Crates | Notes |
| --- | --- | --- |
| General Parsing | [`nom`][c~nom~docs]⮳{{hi:nom}}, [`pest`][c~pest~docs]⮳{{hi:pest}}, [`lalrpop`][c~lalrpop~docs]⮳{{hi:lalrpop}}, [`chumsky`][c~chumsky~docs]⮳{{hi:chumsky}}, [`combine`][c~combine~docs]⮳{{hi:combine}} | [`nom`][c~nom~docs]⮳{{hi:nom}} is good for binary formats and complex grammars. [`pest`][c~pest~docs]⮳{{hi:pest}} is easy for text-based grammars. [`lalrpop`][c~lalrpop~docs]⮳{{hi:lalrpop}} generates efficient parsers. [`chumsky`][c~chumsky~docs]⮳{{hi:chumsky}} is a parser combinator library, [`combine`][c~combine~docs]⮳{{hi:combine}} is another popular choice. |
| Lexing/Tokenization | [`logos`][c~logos~docs]⮳{{hi:logos}}, [`lexical`][c~lexical~docs]⮳{{hi:lexical}}, [`regex`][c~regex~docs]⮳{{hi:regex}} | [`logos`][c~logos~docs]⮳{{hi:logos}} generates fast lexers. [`lexical`][c~lexical~docs]⮳{{hi:lexical}} is another lexer generator. [`regex`][c~regex~docs]⮳{{hi:regex}} is useful for simpler tokenization or text processing. |
| Parser Generators | [`lalrpop`][c~lalrpop~docs]⮳{{hi:lalrpop}}, [`pest`][c~pest~docs]⮳{{hi:pest}} | These tools define the grammar and generate the parser code. |
| Combinator Parsers | [`nom`][c~nom~docs]⮳{{hi:nom}}, [`chumsky`][c~chumsky~docs]⮳{{hi:chumsky}}, [`combine`][c~combine~docs]⮳{{hi:combine}} | These libraries provide functions that can be combined to build more complex parsers. |
| Abstract Syntax Trees (AST) | Often implemented in custom structs | Parsers usually produce an AST, which represents the structure of the parsed input. You'll often define your own structs for this. |
| Serialization/Deserialization | [`serde`][c~serde~docs]⮳{{hi:serde}} | If you need to serialize or deserialize an AST, [`serde`][c~serde~docs]⮳{{hi:serde}} is the standard choice. |

## Code Examples

{{#include parsing.incl.md}}

## Related Topics

- [[argument_parsing | Argument Parsing]].
- [[parser-implementations | Parser Implementations]].
- [[text-processing | Text Processing]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[review](https://github.com/john-cd/rust_howto/issues/952)

</div>
