<!-- TODO Reviewed ??-??-2026. -->

# Encoding and Serialization

[![cat~encoding][cat~encoding~badge]][cat~encoding]{{hi:Encoding}}{{hi:Serialization}}

Choose a representation that matches the data's audience and compatibility needs. Text encodings turn bytes into text; formats such as CSV and spreadsheets organize records; Serde connects Rust types to supported data formats.

| Task | Guide | Crates |
| --- | --- | --- |
| Read or write text in a known character encoding | [Character Encoding](string_encoding.md) | `std`, `encoding_rs`, `encoding` |
| Read, filter, or write delimited records | [CSV Processing](csv.md) | `csv`, `serde` |
| Read or write spreadsheet workbooks | [Excel Spreadsheets](excel.md) | `calamine`, `rust_xlsxwriter` |
| Serialize Rust types to a supported format | [Serialization with `serde`](serde.md) | `serde` and a format crate |

Use a format-specific library when one is available. Before persisting or exchanging data, decide how readers handle unknown fields, schema changes, invalid input, and numeric precision.

## Character Encoding

Rust `String` and `&str` contain valid UTF-8. For other encodings, see [Character Encoding](string_encoding.md).

## Delimited Records

The [CSV recipes](csv.md) cover typed rows, alternate delimiters, filtering, invalid values, and writing records.

## Spreadsheets

The [Excel examples](excel.md) read existing workbooks with `calamine` and create `.xlsx` files with `rust_xlsxwriter`.

## Serde

Serde is a data model and trait framework; format support comes from separate crates. See [Serialization with `serde`](serde.md) and [Serde's data formats](https://serde.rs/#data-formats).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
