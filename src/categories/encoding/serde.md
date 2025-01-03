# Serialization

{{#include serde.incl.md}}

[![serde][c-serde-badge]][c-serde]{{hi:serde}}
[![serde-crates.io][c-serde-crates.io-badge]][c-serde-crates.io]
[![serde-github][c-serde-github-badge]][c-serde-github]
[![serde-lib.rs][c-serde-lib.rs-badge]][c-serde-lib.rs]

De facto standard serialization library. Use in conjunction with sub-crates like serde_json for the specific format that you are using.

```rust,editable
{{#include ../../../crates/ex/categories/efghijkl/tests/encoding/serde.rs:example}}
```

## Serialize JSON {#serde-json2}

[![serde_json][c-serde_json-badge]][c-serde_json]{{hi:serde_json}}
[![serde_json-crates.io][c-serde_json-crates.io-badge]][c-serde_json-crates.io]
[![serde_json-github][c-serde_json-github-badge]][c-serde_json-github]
[![serde_json-lib.rs][c-serde_json-lib.rs-badge]][c-serde_json-lib.rs]

```rust,editable
{{#include ../../../crates/ex/categories/efghijkl/tests/encoding/serde_json.rs:example}}
```

## See also

### `monostate` {#monostate}

[![monostate][c-monostate-badge]][c-monostate]{{hi:monostate}}
[![monostate-crates.io][c-monostate-crates.io-badge]][c-monostate-crates.io]
[![monostate-github][c-monostate-github-badge]][c-monostate-github]
[![monostate-lib.rs][c-monostate-lib.rs-badge]][c-monostate-lib.rs]

This library implements a type macro for a zero-sized type that is Serde deserializable only from one specific value.

```rust,editable
{{#include ../../../crates/ex/categories/efghijkl/tests/encoding/monostate.rs:example}}
```

### `serde-ignored` {#serde-ignored}

[![serde-ignored][c-serde_ignored-badge]][c-serde_ignored]{{hi:serde-ignored}}
[![serde-ignored-crates.io][c-serde_ignored-crates.io-badge]][c-serde_ignored-crates.io]
[![serde-ignored-github][c-serde_ignored-github-badge]][c-serde_ignored-github]
[![serde-ignored-lib.rs][c-serde_ignored-lib.rs-badge]][c-serde_ignored-lib.rs]

```rust,editable
{{#include ../../../crates/ex/categories/efghijkl/tests/encoding/serde_ignored.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[serde: write (P1)](https://github.com/john-cd/rust_howto/issues/352)

dedupe JSON with complex.md
[Supported formats](https://docs.rs/serde/latest/serde/#data-formats)

</div>
