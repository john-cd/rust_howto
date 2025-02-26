# YAML Parsing

{{#include yaml.incl.md}}

## Parse YAML with `serde_yml` {#serde_yml}

[![serde_yml-website][c-serde_yml-website-badge]][c-serde_yml-website] [![serde_yml][c-serde_yml-badge]][c-serde_yml] [![serde_yml-crates.io][c-serde_yml-crates.io-badge]][c-serde_yml-crates.io] [![serde_yml-github][c-serde_yml-github-badge]][c-serde_yml-github] [![serde_yml-lib.rs][c-serde_yml-lib.rs-badge]][c-serde_yml-lib.rs]{{hi:serde_yml}}{{hi:Serialization}}{{hi:Yaml}}{{hi:Serde}} [![cat-encoding][cat-encoding-badge]][cat-encoding]{{hi:Encoding}} [![cat-parser-implementations][cat-parser-implementations-badge]][cat-parser-implementations]{{hi:Parser implementations}}

[`serde_yml`][c-serde_yml]⮳{{hi:serde_yml}} is a robust Rust library that simplifies the serialization and deserialization of Rust data structures to and from YAML format using the widely-used [`serde`][c-serde]⮳{{hi:serde}} framework.

```rust,editable
{{#include ../../../crates/cats/parser_implementations/tests/yaml/serde_yml.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO P2 write

[`serde_yml`][c-serde_yml]⮳{{hi:serde_yml}}, [`yaml-rust`][c-yaml_rust]⮳{{hi:yaml-rust}}

[`serde_yml`][c-serde_yml]⮳{{hi:serde_yml}} integrates well with [`serde`][c-serde]⮳{{hi:serde}}.

</div>
