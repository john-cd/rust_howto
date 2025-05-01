# TOML

{{#include toml.incl.md}}

[`toml`][c-toml]⮳{{hi:toml}} is the primary crate.

## `toml` {#toml}

[![toml][c-toml-badge]][c-toml] [![toml-crates.io][c-toml-crates.io-badge]][c-toml-crates.io] [![toml-github][c-toml-github-badge]][c-toml-github] [![toml-lib.rs][c-toml-lib.rs-badge]][c-toml-lib.rs]{{hi:toml}}{{hi:Encoding}}{{hi:toml}} [![cat-config][cat-config-badge]][cat-config]{{hi:Configuration}} [![cat-encoding][cat-encoding-badge]][cat-encoding]{{hi:Encoding}} [![cat-parser-implementations][cat-parser-implementations-badge]][cat-parser-implementations]{{hi:Parser implementations}} [![cat-parsing][cat-parsing-badge]][cat-parsing]{{hi:Parsing tools}}

[`toml`][c-toml]⮳{{hi:toml}} is a native Rust encoder and decoder of TOML-formatted files and [streams][p-streams]. Provides implementations of the standard Serialize/Deserialize [traits][p-traits] for TOML data to facilitate deserializing and serializing Rust structures.

```rust,editable
{{#include ../../../crates/cats/parser_implementations/tests/toml/toml.rs:example}}
```

## `toml_edit` {#toml-edit}

[![toml_edit][c-toml_edit-badge]][c-toml_edit] [![toml_edit-crates.io][c-toml_edit-crates.io-badge]][c-toml_edit-crates.io] [![toml_edit-github][c-toml_edit-github-badge]][c-toml_edit-github] [![toml_edit-lib.rs][c-toml_edit-lib.rs-badge]][c-toml_edit-lib.rs]{{hi:toml_edit}}{{hi:Encoding}}{{hi:Toml}} [![cat-config][cat-config-badge]][cat-config]{{hi:Configuration}} [![cat-encoding][cat-encoding-badge]][cat-encoding]{{hi:Encoding}} [![cat-parser-implementations][cat-parser-implementations-badge]][cat-parser-implementations]{{hi:Parser implementations}} [![cat-parsing][cat-parsing-badge]][cat-parsing]{{hi:Parsing tools}}

[`toml_edit`][c-toml_edit]⮳{{hi:toml_edit}} is a format-preserving TOML parser.

```rust,editable
{{#include ../../../crates/cats/parser_implementations/tests/toml/toml_edit.rs:example}}
```

## `basic_toml` {#basic-toml}

[![basic-toml][c-basic_toml-badge]][c-basic_toml] [![basic-toml-crates.io][c-basic_toml-crates.io-badge]][c-basic_toml-crates.io] [![basic-toml-github][c-basic_toml-github-badge]][c-basic_toml-github] [![basic-toml-lib.rs][c-basic_toml-lib.rs-badge]][c-basic_toml-lib.rs]{{hi:basic-toml}}{{hi:Toml}}{{hi:Serde}} [![cat-config][cat-config-badge]][cat-config]{{hi:Configuration}} [![cat-parser-implementations][cat-parser-implementations-badge]][cat-parser-implementations]{{hi:Parser implementations}} [![cat-encoding][cat-encoding-badge]][cat-encoding]{{hi:Encoding}}

[`basic_toml`][c-basic_toml]⮳{{hi:basic_toml}} is a minimal TOML library with few dependencies

```rust,editable
{{#include ../../../crates/cats/parser_implementations/tests/toml/basic_toml.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[toml: write](https://github.com/john-cd/rust_howto/issues/444)
</div>
