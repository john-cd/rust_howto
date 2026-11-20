<!-- TODO Reviewed ??-??-2026. -->

# Encoding and Serialization

[![cat~encoding][cat~encoding~badge]][cat~encoding]{{hi:Encoding}}{{hi:Serialization}}

Choose a representation that matches the data's audience and compatibility needs. Text encodings turn bytes into text; CSV and spreadsheets organize records; Serde connects Rust types to supported data formats; binary encoders define wire formats for persistence and communication.

| Task | Guide | Crates |
| --- | --- | --- |
| Read or write text in a known character encoding | [Character Encoding](string_encoding.md) | `std`, `encoding_rs`, `encoding` |
| Read, filter, or write delimited records | [CSV Processing](csv.md) | `csv`, `serde` |
| Read or write spreadsheet workbooks | [Excel Spreadsheets](excel.md) | `calamine`, `rust_xlsxwriter` |
| Serialize Rust types to a supported format | [Serialization with `serde`](serde.md) | `serde` and a format crate |
| Read structured JSON or TOML values | [Structured Data](complex_encoding.md) | `serde_json`, `toml`, `byteorder` |
| Convert between numeric types and byte layouts | [Typecasts and Byte Layout](typecasts.md) | `TryFrom`, `bytemuck`, `zerocopy`, `byteorder` |
| Encode data with a separate schema | [Binary Encoders](binary_encoders.md) | `prost`, `capnp`, `flatbuffers` |
| Encode typed values without a separate schema | [Binary Encoders Without External Schemas](no_external_schema.md) | `postcard`, `rmp-serde`, `ciborium`, `rkyv` |

Use a format-specific library when one is available. Before persisting or exchanging data, decide how readers handle unknown fields, schema changes, invalid input, and numeric precision.

## Character Encoding

Rust `String` and `&str` contain valid UTF-8. For other encodings, see [Character Encoding](string_encoding.md).

## Delimited Records

The [CSV recipes](csv.md) cover typed rows, alternate delimiters, filtering, invalid values, and writing records.

## Spreadsheets

The [Excel examples](excel.md) read existing workbooks with `calamine` and create `.xlsx` files with `rust_xlsxwriter`.

## Serde

Serde is a data model and trait framework; format support comes from separate crates. See [Serialization with `serde`](serde.md) and [Serde's data formats](https://serde.rs/#data-formats).

## Structured Data and Type Conversions

[Structured Data](complex_encoding.md) compares JSON values, TOML configuration data, and endian-aware integer fields. [Typecasts and Byte Layout](typecasts.md) distinguishes primitive casts from checked conversions and explains safe byte views.

## Binary Formats

Use [schema-first binary encoders](binary_encoders.md) when a separate schema is part of the wire contract. [Binary Encoders Without External Schemas](no_external_schema.md) compares formats that use Rust types directly, including their type and compatibility requirements.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
