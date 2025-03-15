# General Programming Language Parsing

{{#include programming_languages.incl.md}}

## Parse JavaScript {#javascript_parsing}

[![swc_ecma_parser][c-swc_ecma_parser-badge]][c-swc_ecma_parser]{{hi:swc_ecma_parser}}
[![swc_ecma_parser-crates.io][c-swc_ecma_parser-crates.io-badge]][c-swc_ecma_parser-crates.io]
[![swc_ecma_parser-github][c-swc_ecma_parser-github-badge]][c-swc_ecma_parser-github]
[![swc_ecma_parser-lib.rs][c-swc_ecma_parser-lib.rs-badge]][c-swc_ecma_parser-lib.rs]

[swc_ecma_parser][c-swc_ecma_parser-crates.io]{{hi:swc_ecma_parser}}⮳ is a feature-complete ECMAScript / TypeScript parser written in Rust.

```rust,editable
{{#include ../../../crates/cats/parser_implementations/tests/javascript/swc_ecma_parser.rs:example}}
```

## Parse SQL {#sql_parsing}

[![sqlparser][c-sqlparser-badge]][c-sqlparser] [![sqlparser-crates.io][c-sqlparser-crates.io-badge]][c-sqlparser-crates.io] [![sqlparser-github][c-sqlparser-github-badge]][c-sqlparser-github] [![sqlparser-lib.rs][c-sqlparser-lib.rs-badge]][c-sqlparser-lib.rs]{{hi:sqlparser}}{{hi:Parser}}{{hi:Sql}}{{hi:Ansi}}{{hi:Lexer}}

[`sqlparser`][c-sqlparser]⮳{{hi:sqlparser}} is a general SQL lexer and parser with support for ANSI SQL:2011.

```rust,editable
{{#include ../../../crates/cats/parser_implementations/tests/programming_languages/sqlparser.rs:example}}
```

### See Also

[`diesel`][c-diesel]⮳{{hi:diesel}} is an ORM that includes SQL parsing.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/951)

## Parse Rust Code {#rust_parsing}

[`syn`][c-syn]⮳{{hi:syn}}, [`quote`][c-quote]⮳{{hi:quote}}

[`syn`][c-syn]⮳{{hi:syn}} parses Rust code into an AST. [`quote`][c-quote]⮳{{hi:quote}} is often used alongside [`syn`][c-syn]⮳{{hi:syn}} for code generation. |

Link to Proc Macros

## WebAssembly (WAT/WASM) {#wasm_parsing}

[`wat`][c-wat]⮳{{hi:wat}}, [`parity-wasm`][c-parity_wasm]⮳{{hi:parity-wasm}}

[`wat`][c-wat]⮳{{hi:wat}} is for parsing WAT (WebAssembly Text Format). [`parity-wasm`][c-parity_wasm]⮳{{hi:parity-wasm}} is a more general WebAssembly tooling library. |

Link to WebAssembly

</div>
