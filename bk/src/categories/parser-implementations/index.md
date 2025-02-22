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
| JSON | [`serde_json`][c-serde_json]⮳{{hi:serde_json}}, [`simd_json`][c-simd_json]⮳{{hi:simd_json}} | [`serde_json`][c-serde_json]⮳{{hi:serde_json}} is the most commonly used. [`simd_json`][c-simd_json]⮳{{hi:simd_json}} is optimized for performance. |
| YAML | [`serde_yaml`][c-serde_yaml]⮳{{hi:serde_yaml}}, [`yaml-rust`][c-yaml_rust]⮳{{hi:yaml-rust}} | [`serde_yaml`][c-serde_yaml]⮳{{hi:serde_yaml}} integrates well with [`serde`][c-serde]⮳{{hi:serde}}. |
| TOML | [`toml`][c-toml]⮳{{hi:toml}}, [`serde_toml`][c-serde_toml]⮳{{hi:serde_toml}} | [`toml`][c-toml]⮳{{hi:toml}} is the primary crate. [`serde_toml`][c-serde_toml]⮳{{hi:serde_toml}} provides [`serde`][c-serde]⮳{{hi:serde}} integration. |
| CSV | [`csv`][c-csv]⮳{{hi:csv}} | A robust and widely used CSV parser. |
| XML | [`quick_xml`][c-quick_xml]⮳{{hi:quick_xml}}, [`roxmltree`][c-roxmltree]⮳{{hi:roxmltree}}, [`minidom`][c-minidom]⮳{{hi:minidom}} | [`quick_xml`][c-quick_xml]⮳{{hi:quick_xml}} is fast for streaming. [`roxmltree`][c-roxmltree]⮳{{hi:roxmltree}} is good for simple parsing. [`minidom`][c-minidom]⮳{{hi:minidom}} builds a DOM tree. |
| HTML | [`scraper`][c-scraper]⮳{{hi:scraper}}, [`select`][c-select]⮳{{hi:select}}, [`kuchiki`][c-kuchiki]⮳{{hi:kuchiki}} | [`scraper`][c-scraper]⮳{{hi:scraper}} uses CSS selectors. [`select`][c-select]⮳{{hi:select}} is another option. [`kuchiki`][c-kuchiki]⮳{{hi:kuchiki}} is a fast and robust HTML parser. |
| Markdown | [`pulldown-cmark`][c-pulldown_cmark]⮳{{hi:pulldown-cmark}}, [`comrak`][c-comrak]⮳{{hi:comrak}} | [`pulldown-cmark`][c-pulldown_cmark]⮳{{hi:pulldown-cmark}} is CommonMark compliant. [`comrak`][c-comrak]⮳{{hi:comrak}} is another popular choice. |
| Regular Expressions | [`regex`][c-regex]⮳{{hi:regex}} | The standard crate for regular expression parsing and matching. |
| INI | [`ini`][c-ini]⮳{{hi:ini}} | For parsing INI configuration files. |
| GraphQL | [`graphql_parser`][c-graphql_parser]⮳{{hi:graphql_parser}}, [`juniper`][c-juniper]⮳{{hi:juniper}} | [`graphql_parser`][c-graphql_parser]⮳{{hi:graphql_parser}} parses GraphQL queries. [`juniper`][c-juniper]⮳{{hi:juniper}} is a GraphQL server implementation and includes parsing capabilities. |
| WebAssembly (WAT/Wasm) | `wat`, [`parity-wasm`][c-parity_wasm]⮳{{hi:parity-wasm}} | [`wat`][c-wat]⮳{{hi:wat}} is for parsing WAT (WebAssembly Text Format). [`parity-wasm`][c-parity_wasm]⮳{{hi:parity-wasm}} is a more general WebAssembly tooling library. |
| SQL | [`sqlparser`][c-sqlparser]⮳{{hi:sqlparser}}, [`diesel`][c-diesel]⮳{{hi:diesel}} | [`sqlparser`][c-sqlparser]⮳{{hi:sqlparser}} is a general SQL parser. [`diesel`][c-diesel]⮳{{hi:diesel}} is an ORM that includes SQL parsing. |
| Rust Code | `syn`, [`quote`][c-quote]⮳{{hi:quote}} | [`syn`][c-syn]⮳{{hi:syn}} parses Rust code into an AST. [`quote`][c-quote]⮳{{hi:quote}} is often used alongside [`syn`][c-syn]⮳{{hi:syn}} for code generation. |

</div>
