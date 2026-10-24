<!-- TODO Reviewed ??-??-2026 -->

# YAML Parsing

{{#include yaml.incl.md}}

## Parse YAML with `serde-saphyr` {#serde-saphyr}

[![serde-saphyr][c~serde-saphyr~docs~badge]][c~serde-saphyr~docs] [![serde-saphyr~crates.io][c~serde-saphyr~crates.io~badge]][c~serde-saphyr~crates.io]{{hi:serde-saphyr}}{{hi:Serialization}}{{hi:Yaml}}{{hi:Serde}} [![cat~encoding][cat~encoding~badge]][cat~encoding]{{hi:Encoding}} [![cat~parser-implementations][cat~parser-implementations~badge]][cat~parser-implementations]{{hi:Parser implementations}}

[`serde_yml`][c~serde_yml~docs]↗ is deprecated and unmaintained. Its documentation lists `noyalib`, `serde-saphyr`, and `yaml-rust2` as maintained migration options. For this typed Serde recipe, use [`serde-saphyr`][c~serde-saphyr~docs]↗: it deserializes directly into Rust types and also provides a serializer. The example below derives `Deserialize` and `Serialize`, then checks that a value survives a YAML round trip.

```rust,editable
{{#include ../../../crates/cats/parser_implementations/examples/yaml/serde_saphyr.rs:example}}
```

## Other Options {#other-options .skip}

Use [`noyalib`][c~noyalib~docs]↗ if you need a YAML value tree or its concrete syntax tree APIs alongside Serde support. For lower-level parsing without a Serde data model, consider [`saphyr`][c~saphyr~docs]↗ or [`yaml-rust2`][c~yaml-rust2~docs]↗.

## Related Topics {#related-topics .skip}

- [Serde][c~serde~docs]↗
- [TOML parsing][ex~parser-implementations~toml]
- [JSON parsing][ex~parser-implementations~serde-json]
- [Configuration files][ex~parser-implementations~configuration]

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
This chapter recommends an actively maintained Serde-based YAML implementation, distinguishes it from lower-level parsers, and demonstrates typed serialization and deserialization with a tested round trip.

</div>
