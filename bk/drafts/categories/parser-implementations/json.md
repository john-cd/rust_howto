# JSON Parsing

{{#include json.incl.md}}

[`serde_json`][c~serde_json~docs]↗{{hi:serde_json}} is the most commonly used crate for JSON Parsing. [`simd-json`][c~simd-json~docs]↗{{hi:simd-json}} is optimized for performance.

## Parse JSON with `serde_json` {#serde-json}

[![serde_json][c~serde_json~docs~badge]][c~serde_json~docs] [![serde_json~crates.io][c~serde_json~crates.io~badge]][c~serde_json~crates.io] [![serde_json~repo][c~serde_json~repo~badge]][c~serde_json~repo] [![serde_json~lib.rs][c~serde_json~lib.rs~badge]][c~serde_json~lib.rs]{{hi:serde_json}}{{hi:Json}}{{hi:Serde}}{{hi:Serialization}} [![cat~parser-implementations][cat~parser-implementations~badge]][cat~parser-implementations]{{hi:Parser implementations}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}} [![cat~encoding][cat~encoding~badge]][cat~encoding]{{hi:Encoding}}

[`serde_json`][c~serde_json~docs]↗{{hi:serde_json}} offers a JSON serialization file format.

```rust,editable
{{#include ../../../crates/cats/parser_implementations/examples/json/serde_json.rs:example}}
```

## Parse JSON with `json5` {#json5}

[![json5][c~json5~docs~badge]][c~json5~docs] [![json5~crates.io][c~json5~crates.io~badge]][c~json5~crates.io] [![json5~repo][c~json5~repo~badge]][c~json5~repo] [![json5~lib.rs][c~json5~lib.rs~badge]][c~json5~lib.rs]{{hi:json5}}{{hi:Serde}}{{hi:Parse}}{{hi:json5}}{{hi:Parser}}{{hi:Json}}

[`json5`][c~json5~docs]↗{{hi:json5}} is a Rust JSON5 serializer and deserializer which speaks [`serde`][c~serde~docs]↗{{hi:serde}}.

```rust,editable
{{#include ../../../crates/cats/parser_implementations/examples/json/json5.rs:example}}
```

## Parse JSON with `simd-json` {#simd-json}

[![simd-json][c~simd-json~docs~badge]][c~simd-json~docs] [![simd-json~crates.io][c~simd-json~crates.io~badge]][c~simd-json~crates.io] [![simd-json~repo][c~simd-json~repo~badge]][c~simd-json~repo] [![simd-json~lib.rs][c~simd-json~lib.rs~badge]][c~simd-json~lib.rs]{{hi:simd-json}}

[`simd-json`][c~simd-json~docs]↗{{hi:simd-json}} is a high-performance JSON parser based on a port of [`simdjson`][simdjson~repo]↗{{hi:simdjson}}.

```rust,editable
{{#include ../../../crates/cats/parser_implementations/examples/json/simd-json.rs:example}}
```

## Related Topics {#related-topics .skip}

FIXME

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[json: include in index.md / SUMMARY.md; write](https://github.com/john-cd/rust_howto/issues/440)
</div>
