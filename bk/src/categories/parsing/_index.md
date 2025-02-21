# Parsing

[![cat-parsing][cat-parsing-badge]][cat-parsing]{{hi:Parsing}}

Create parsers for binary and text formats. Format-specific parsers belong in other, more specific categories.

{{#include parsing.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[ P2 review](https://github.com/john-cd/rust_howto/issues/952)

| Topic | Rust Crates | Notes |
| --- | --- | --- |
| General Parsing | `nom`, `pest`, `lalrpop`, `chumsky`, `combine` | `nom` is good for binary formats and complex grammars. `pest` is easy for text-based grammars. `lalrpop` generates efficient parsers. `chumsky` is a parser combinator library, `combine` is another popular choice. |
| Lexing/Tokenization | `logos`, `lexical`, `regex` | `logos` generates fast lexers. `lexical` is another lexer generator. `regex` is useful for simpler tokenization or text processing. |
| Parser Generators | `lalrpop`, `pest` | These tools define the grammar and generate the parser code. |
| Combinator Parsers | `nom`, `chumsky`, `combine` | These libraries provide functions that can be combined to build more complex parsers. |
| Error Handling | | Good error messages are crucial. Consider crates that provide helpful error reporting. |
| Abstract Syntax Trees (AST) | Often implemented in custom structs | Parsers usually produce an AST, which represents the structure of the parsed input. You'll often define your own structs for this. |
| Serialization/Deserialization | `serde` | If you need to serialize or deserialize the AST, `serde` is the standard library. |

</div>
