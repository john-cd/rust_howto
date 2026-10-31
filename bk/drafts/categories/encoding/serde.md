<!-- TODO Reviewed ??-??-2026 -->

# Serialization

{{#include serde.incl.md}}

Serde is a framework for converting Rust data to and from a shared data model. Format crates such as `serde_json` implement that model for a specific wire format, so a type can often be used with multiple formats without changing its data definition.

## Serialization with `serde` {#serde}

{{#badges serde}}

[`serde`][c~serde~docs]↗{{hi:serde}} is the de facto standard serialization library. Use in conjunction with sub-crates like [`serde_json`][c~serde_json~docs]↗{{hi:serde_json}} for the specific format that you are using.

Enable the `derive` feature and derive `Serialize` and `Deserialize` for ordinary structs and enums. Use Serde attributes such as `rename`, `default`, and `skip` when the wire representation differs from the Rust API. For custom behavior, implement the traits directly. Treat field names, enum representation, and omission rules as part of the format contract when data is persisted or exchanged between versions.

```rust,editable
{{#include ../../../crates/cats/encoding/examples/serde/serde.rs:example}}
```

## (De)serialize JSON {#serde-json2}

{{#badges serde_json}}

[`serde-json`][c~serde_json~docs]↗{{hi:serde-json}}

Use `serde_json::to_string` or `to_vec` to serialize and `from_str` or `from_slice` to deserialize. Deserialization returns an error for invalid input; validate application-specific constraints separately after parsing.

```rust,editable
{{#include ../../../crates/cats/encoding/examples/serde/serde_json.rs:example}}
```

## Handle Unknown Fields When Deserializing, with `serde-ignored` {#serde-ignored}

[![serde-ignored][c~serde_ignored~docs~badge]][c~serde_ignored~docs]{{hi:serde-ignored}}
[![serde-ignored~crates.io][c~serde_ignored~crates.io~badge]][c~serde_ignored~crates.io]
[![serde-ignored~repo][c~serde_ignored~repo~badge]][c~serde_ignored~repo]
[![serde-ignored~lib.rs][c~serde_ignored~lib.rs~badge]][c~serde_ignored~lib.rs]

[`serde-ignored`][c~serde_ignored~docs]↗{{hi:serde-ignored}}

Capture names of fields that the target type does not recognize while deserializing. This can help configuration tools report misspelled or obsolete keys instead of silently overlooking them.

```rust,editable
{{#include ../../../crates/cats/encoding/examples/serde/serde_ignored.rs:example}}
```

## `monostate` {#monostate}

{{#badges monostate}}

The [`monostate`][c~monostate~docs]↗{{hi:monostate}} library implements a type macro for a zero-sized type that is Serde deserializable only from one specific value.

Use this for a protocol field that must contain one exact sentinel value, for example a tagged marker. It does not replace validation for the rest of the input.

```rust,editable
{{#include ../../../crates/cats/encoding/examples/serde/monostate.rs:example}}
```

## Choosing a Format

Serde supports many formats through separate crates. See [Serde's data formats](https://serde.rs/#data-formats) and the book's [CSV](csv.md), [binary encoders](_binary_encoders.md), and [complex encodings](complex_encoding.md) chapters. Choose and document a format based on interoperability, schema evolution, human readability, and size requirements.

## Further Reading {#further-reading .skip}

- [`serde_with`][c~serde_with~lib.rs]↗ for adapters and common serialization patterns.

## Related Topics {#related-topics .skip}

- [JSON](../parser-implementations/json.md).
- [CSV](csv.md).
- [Binary encoders](_binary_encoders.md).
- [Encoding category](index.md).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
