# General Programming Language Parsing

{{#include programming_languages.incl.md}}

## Parse JavaScript {#javascript_parsing}

[![swc_ecma_parser][c~swc_ecma_parser~docs~badge]][c~swc_ecma_parser~docs]{{hi:swc_ecma_parser}}
[![swc_ecma_parser~crates.io][c~swc_ecma_parser~crates.io~badge]][c~swc_ecma_parser~crates.io]
[![swc_ecma_parser~github][c~swc_ecma_parser~github~badge]][c~swc_ecma_parser~github]
[![swc_ecma_parser~lib.rs][c~swc_ecma_parser~lib.rs~badge]][c~swc_ecma_parser~lib.rs]

[swc_ecma_parser][c~swc_ecma_parser~crates.io]{{hi:swc_ecma_parser}}↗ is a feature-complete ECMAScript / TypeScript parser written in Rust.

```rust,editable
{{#include ../../../crates/cats/parser_implementations/examples/javascript/swc_ecma_parser.rs:example}}
```

## Parse SQL {#sql_parsing}

[![sqlparser][c~sqlparser~docs~badge]][c~sqlparser~docs] [![sqlparser~crates.io][c~sqlparser~crates.io~badge]][c~sqlparser~crates.io] [![sqlparser~github][c~sqlparser~github~badge]][c~sqlparser~github] [![sqlparser~lib.rs][c~sqlparser~lib.rs~badge]][c~sqlparser~lib.rs]{{hi:sqlparser}}{{hi:Parser}}{{hi:Sql}}{{hi:Ansi}}{{hi:Lexer}}

[`sqlparser`][c~sqlparser~docs]↗{{hi:sqlparser}} is a general SQL lexer and parser with support for ANSI SQL:2011.

```rust,editable
{{#include ../../../crates/cats/parser_implementations/examples/programming_languages/sqlparser.rs:example}}
```

See also [`diesel`][c~diesel~docs]↗{{hi:diesel}}, an ORM that includes SQL parsing.

## Parse Rust Code {#rust_parsing}

[`syn`][c~syn~docs]↗{{hi:syn}} parses Rust code into an AST. [`quote`][c~quote~docs]↗{{hi:quote}} is often used alongside [`syn`][c~syn~docs]↗{{hi:syn}} for code generation.

See [[write_proc_macros | Write Proc Macros]].

## Parse WebAssembly (WAT/WASM) {#wasm_parsing}

[`wat`][c~wat~docs]↗{{hi:wat}} parses WAT (WebAssembly Text Format). [`parity-wasm`][c~parity_wasm~docs]↗{{hi:parity-wasm}} is a more general WebAssembly tooling library.

Refer to the [[wasm | WASM]] chapter.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/951)

- [Reval — Rust expression parser](https://lib.rs/crates/reval)

</div>
