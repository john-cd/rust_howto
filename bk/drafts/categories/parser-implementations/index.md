<!-- TODO Reviewed ??-??-2026 -->

# Parser Implementations

[![cat~parser-implementations][cat~parser-implementations~badge]][cat~parser-implementations]{{hi:Parser implementations}}

This index links format-specific parser topics and their runnable examples. Use it when you need to read an established data, document, or language format. For parser combinators, parser generators, lexing, and custom grammars, see [Parsing](../parsing/index.md).

## Structured Data and Configuration

### [INI](ini.md)

{{#include ini.incl.md}}

### [JSON](json.md)

{{#include json.incl.md}}

### [TOML](toml.md)

{{#include toml.incl.md}}

### [YAML](yaml.md)

{{#include yaml.incl.md}}

## Markup and Documents

### [HTML and CSS](html.md)

{{#include html.incl.md}}

### [Markdown](markdown.md)

{{#include markdown.incl.md}}

### [XML](xml.md)

{{#include xml.incl.md}}

## Programming Languages

### [Programming Language Parsers](programming_languages.md)

{{#include programming_languages.incl.md}}

## Other Text and Query Formats

| Format | Crates | Notes |
| --- | --- | --- |
| CSV | [`csv`][c~csv~docs]↗{{hi:csv}} | Parse delimited records with a CSV-aware reader that handles quoting and escaping. |
| GraphQL | [`async-graphql-parser`][c~async-graphql-parser~docs]↗{{hi:async-graphql-parser}}, [`graphql-parser`][c~graphql-parser~docs]↗{{hi:graphql-parser}} | `async-graphql-parser` is the parser used by [`async-graphql`][c~async-graphql~docs]; `graphql-parser` is an independent parser with query and schema ASTs. For schema execution, see [`async-graphql`][c~async-graphql~docs] or [`juniper`][c~juniper~docs]. |
| Regular expressions | [`regex`][c~regex~docs]↗{{hi:regex}} | Match text with regular-expression patterns. For nested or structurally rich syntax, use a grammar-based parser. |

## Related Topics

- [Configuration](../config/configuration.md) covers loading application settings from files and other sources.
- [Text Processing](../text-processing/index.md) covers string operations and regular-expression use.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
