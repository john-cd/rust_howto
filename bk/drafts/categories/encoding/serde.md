# Serialization

{{#include serde.incl.md}}

## Serialization with `serde` {#serde}

[![serde][c~serde~docs~badge]][c~serde~docs]{{hi:serde}}
[![serde~crates.io][c~serde~crates.io~badge]][c~serde~crates.io]
[![serde~github][c~serde~github~badge]][c~serde~github]
[![serde~lib.rs][c~serde~lib.rs~badge]][c~serde~lib.rs]

[`serde`][c~serde~docs]↗{{hi:serde}} is the de facto standard serialization library. Use in conjunction with sub-crates like [`serde_json`][c~serde_json~docs]↗{{hi:serde_json}} for the specific format that you are using.

```rust,editable
{{#include ../../../crates/cats/encoding/examples/serde/serde.rs:example}}
```

## (De)serialize JSON {#serde-json2}

[![serde_json][c~serde_json~docs~badge]][c~serde_json~docs]{{hi:serde_json}}
[![serde_json~crates.io][c~serde_json~crates.io~badge]][c~serde_json~crates.io]
[![serde_json~github][c~serde_json~github~badge]][c~serde_json~github]
[![serde_json~lib.rs][c~serde_json~lib.rs~badge]][c~serde_json~lib.rs]

[`serde-json`][c~serde_json~docs]↗{{hi:serde-json}}

```rust,editable
{{#include ../../../crates/cats/encoding/examples/serde/serde_json.rs:example}}
```

## Handle Unknown Fields When Deserializing, with `serde-ignored` {#serde-ignored}

[![serde-ignored][c~serde_ignored~docs~badge]][c~serde_ignored~docs]{{hi:serde-ignored}}
[![serde-ignored~crates.io][c~serde_ignored~crates.io~badge]][c~serde_ignored~crates.io]
[![serde-ignored~github][c~serde_ignored~github~badge]][c~serde_ignored~github]
[![serde-ignored~lib.rs][c~serde_ignored~lib.rs~badge]][c~serde_ignored~lib.rs]

[`serde-ignored`][c~serde_ignored~docs]↗{{hi:serde-ignored}}

```rust,editable
{{#include ../../../crates/cats/encoding/examples/serde/serde_ignored.rs:example}}
```

## `monostate` {#monostate}

[![monostate][c~monostate~docs~badge]][c~monostate~docs]{{hi:monostate}}
[![monostate~crates.io][c~monostate~crates.io~badge]][c~monostate~crates.io]
[![monostate~github][c~monostate~github~badge]][c~monostate~github]
[![monostate~lib.rs][c~monostate~lib.rs~badge]][c~monostate~lib.rs]

The [`monostate`][c~monostate~docs]↗{{hi:monostate}} library implements a type macro for a zero-sized type that is Serde deserializable only from one specific value.

```rust,editable
{{#include ../../../crates/cats/encoding/examples/serde/monostate.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[serde: write](https://github.com/john-cd/rust_howto/issues/352)
dedupe [JSON][p~json] with complex.md.
[Supported Formats][c~serde~supported-formats]↗.

- [`eserde`: Don't stop at the first deserialization error | Mainmatter][blog~mainmatter-eserde]↗.
- [`serde_with` — Rust data encoding library][c~serde_with~lib.rs]↗.
- [`serde_arrow` — Rust data encoding library][c~serde_arrow~lib.rs]↗.
- [`serde-encrypted-value` — Rust data encoding library][c~serde-encrypted-value~lib.rs]↗.

- [`validator`][c~validator~lib.rs]↗: Common validation functions (email, url, length, …) and trait - to be used with `validator_derive`.

</div>
