# Parser Implementations

[![cat-parser-implementations][cat-parser-implementations-badge]][cat-parser-implementations]{{hi:Parser implementations}}

Parsers implemented for particular formats or languages.

{{#include parser_implementations.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[parser-implementations/_index: add other sub-chapters above (P2)](https://github.com/john-cd/rust_howto/issues/447)

| Format/Language | Rust Crates | Notes |
| --- | ---|---|
| JSON | `serde_json`, `simd_json` | `serde_json` is the most commonly used. `simd_json` is optimized for performance. |
| YAML | `serde_yaml`, `yaml-rust` | `serde_yaml` integrates well with `serde`. |
| TOML | `toml`, `serde_toml` | `toml` is the primary crate. `serde_toml` provides `serde` integration. |
| CSV | `csv` | A robust and widely used CSV parser. |
| XML | `quick_xml`, `roxmltree`, `minidom` | `quick_xml` is fast for streaming. `roxmltree` is good for simple parsing. `minidom` builds a DOM tree. |
| HTML | `scraper`, `select`, `kuchiki` | `scraper` uses CSS selectors. `select` is another option. `kuchiki` is a fast and robust HTML parser. |
| Markdown | `pulldown-cmark`, `comrak` | `pulldown-cmark` is CommonMark compliant. `comrak` is another popular choice. |
| Regular Expressions | `regex` | The standard crate for regular expression parsing and matching. |
| INI | `ini` | For parsing INI configuration files. |
| GraphQL | `graphql_parser`, `juniper` | `graphql_parser` parses GraphQL queries. `juniper` is a GraphQL server implementation and includes parsing capabilities. |
| WebAssembly (WAT/Wasm) | `wat`, `parity-wasm` | `wat` is for parsing WAT (WebAssembly Text Format). `parity-wasm` is a more general WebAssembly tooling library. |
| SQL | `sqlparser`, `diesel` | `sqlparser` is a general SQL parser. `diesel` is an ORM that includes SQL parsing. |
| Rust Code | `syn`, `quote` | `syn` parses Rust code into an AST. `quote` is often used alongside `syn` for code generation. |

</div>
