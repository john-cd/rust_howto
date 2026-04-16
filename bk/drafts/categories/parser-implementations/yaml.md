# YAML Parsing

{{#include yaml.incl.md}}

## Parse YAML with `serde_yml` {#serde_yml}

[![serde_yml~website][c~serde_yml~website~badge]][c~serde_yml~website] [![serde_yml][c~serde_yml~docs~badge]][c~serde_yml~docs] [![serde_yml~crates.io][c~serde_yml~crates.io~badge]][c~serde_yml~crates.io] [![serde_yml~repo][c~serde_yml~repo~badge]][c~serde_yml~repo] [![serde_yml~lib.rs][c~serde_yml~lib.rs~badge]][c~serde_yml~lib.rs]{{hi:serde_yml}}{{hi:Serialization}}{{hi:Yaml}}{{hi:Serde}} [![cat~encoding][cat~encoding~badge]][cat~encoding]{{hi:Encoding}} [![cat~parser-implementations][cat~parser-implementations~badge]][cat~parser-implementations]{{hi:Parser implementations}}

[`serde_yml`][c~serde_yml~docs]↗{{hi:serde_yml}} is a robust Rust library that simplifies the serialization and deserialization of Rust data structures to and from YAML format using the widely-used [`serde`][c~serde~docs]↗{{hi:serde}} framework.

```rust,editable
{{#include ../../../crates/cats/parser_implementations/examples/yaml/serde_yml.rs:example}}
```

## Other Options {#other-options .skip}

Consider [`yaml-rust`][c~yaml-rust~docs]↗{{hi:yaml-rust}}. 'serde_yaml' is deprecated.

## Related Topics {#related-topics .skip}

FIXME

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/1186)

- [saphyr][c~saphyr~repo]↗: A set of crates dedicated to parsing YAML.

</div>
