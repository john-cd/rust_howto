# TOML

{{#include toml.incl.md}}

[`toml`][c~toml~docs]↗{{hi:toml}} is the primary crate.

## `toml` {#toml}

[![toml][c~toml~docs~badge]][c~toml~docs] [![toml~crates.io][c~toml~crates.io~badge]][c~toml~crates.io] [![toml~repo][c~toml~repo~badge]][c~toml~repo] [![toml~lib.rs][c~toml~lib.rs~badge]][c~toml~lib.rs]{{hi:toml}}{{hi:Encoding}}{{hi:toml}} [![cat~config][cat~config~badge]][cat~config]{{hi:Configuration}} [![cat~encoding][cat~encoding~badge]][cat~encoding]{{hi:Encoding}} [![cat~parser-implementations][cat~parser-implementations~badge]][cat~parser-implementations]{{hi:Parser implementations}} [![cat~parsing][cat~parsing~badge]][cat~parsing]{{hi:Parsing tools}}

[`toml`][c~toml~docs]↗{{hi:toml}} is a native Rust encoder and decoder of TOML-formatted files and [streams][p~streams]. Provides implementations of the standard Serialize/Deserialize [traits][p~traits] for TOML data to facilitate deserializing and serializing Rust structures.

```rust,editable
{{#include ../../../crates/cats/parser_implementations/examples/toml/toml.rs:example}}
```

## `toml_edit` {#toml-edit}

[![toml_edit][c~toml_edit~docs~badge]][c~toml_edit~docs] [![toml_edit~crates.io][c~toml_edit~crates.io~badge]][c~toml_edit~crates.io] [![toml_edit~repo][c~toml_edit~repo~badge]][c~toml_edit~repo] [![toml_edit~lib.rs][c~toml_edit~lib.rs~badge]][c~toml_edit~lib.rs]{{hi:toml_edit}}{{hi:Encoding}}{{hi:Toml}} [![cat~config][cat~config~badge]][cat~config]{{hi:Configuration}} [![cat~encoding][cat~encoding~badge]][cat~encoding]{{hi:Encoding}} [![cat~parser-implementations][cat~parser-implementations~badge]][cat~parser-implementations]{{hi:Parser implementations}} [![cat~parsing][cat~parsing~badge]][cat~parsing]{{hi:Parsing tools}}

[`toml_edit`][c~toml_edit~docs]↗{{hi:toml_edit}} is a format-preserving TOML parser.

```rust,editable
{{#include ../../../crates/cats/parser_implementations/examples/toml/toml_edit.rs:example}}
```

## `basic-toml` {#basic-toml}

[![basic-toml][c~basic-toml~docs~badge]][c~basic-toml~docs] [![basic-toml~crates.io][c~basic-toml~crates.io~badge]][c~basic-toml~crates.io] [![basic-toml~repo][c~basic-toml~repo~badge]][c~basic-toml~repo] [![basic-toml~lib.rs][c~basic-toml~lib.rs~badge]][c~basic-toml~lib.rs]{{hi:basic-toml}}{{hi:Toml}}{{hi:Serde}} [![cat~config][cat~config~badge]][cat~config]{{hi:Configuration}} [![cat~parser-implementations][cat~parser-implementations~badge]][cat~parser-implementations]{{hi:Parser implementations}} [![cat~encoding][cat~encoding~badge]][cat~encoding]{{hi:Encoding}}

[`basic-toml`][c~basic-toml~docs]↗{{hi:basic-toml}} is a minimal TOML library with few dependencies

```rust,editable
{{#include ../../../crates/cats/parser_implementations/examples/toml/basic_toml.rs:example}}
```

## Related Topics {#related-topics .skip}

FIXME

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[toml: write](https://github.com/john-cd/rust_howto/issues/444)

[![taplo~website][c~taplo~website~badge]][c~taplo~website] [![taplo][c~taplo~docs~badge]][c~taplo~docs] [![taplo~crates.io][c~taplo~crates.io~badge]][c~taplo~crates.io] [![taplo~repo][c~taplo~repo~badge]][c~taplo~repo] [![taplo~lib.rs][c~taplo~lib.rs~badge]][c~taplo~lib.rs]{{hi:taplo}}{{hi:Parser}}{{hi:Toml}}{{hi:Linter}}{{hi:Formatter}} [![cat~parser-implementations][cat~parser-implementations~badge]][cat~parser-implementations]{{hi:Parser implementations}} [![cat~parsing][cat~parsing~badge]][cat~parsing]{{hi:Parsing tools}}

A TOML parser, analyzer and formatter library.

</div>
