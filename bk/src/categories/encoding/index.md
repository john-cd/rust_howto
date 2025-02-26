# Encoding

[![cat-encoding][cat-encoding-badge]][cat-encoding]{{hi:Encoding}}

Encoding and/or decoding{{hi:Decoding}} data from one data format to another.

## Character Sets

{{#include string_encoding.incl.md}}

## CSV Processing

{{#include csv.incl.md}}

## Structured Data (Complex Encoding)

{{#include complex_encoding.incl.md}}

## Serde

{{#include serde.incl.md}}

## Typecasts

{{#include typecasts.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">

## Binary Encoders

{{#include _binary_encoders.incl.md}}

## No external schemas

{{#include no_external_schema.incl.md}}

[P1 review](https://github.com/john-cd/rust_howto/issues/929)

Working with Binary Data: Recipes for reading and writing raw binary data, handling bitwise operations, and working with byte buffers.

## Character Encoding

[`encoding`][c-encoding]⮳{{hi:encoding}}: A comprehensive crate for working with various character encodings (UTF-8, UTF-16, Latin-1, etc.). A good general-purpose choice.
Base64:

[`base64`][c-base64]⮳{{hi:base64}}: A widely used crate for Base64 encoding and decoding.

## URL Encoding

[![percent-encoding][c-percent_encoding-badge]][c-percent_encoding] [![percent-encoding-crates.io][c-percent_encoding-crates.io-badge]][c-percent_encoding-crates.io] [![percent-encoding-github][c-percent_encoding-github-badge]][c-percent_encoding-github] [![percent-encoding-lib.rs][c-percent_encoding-lib.rs-badge]][c-percent_encoding-lib.rs]{{hi:percent-encoding}}

[`percent-encoding`][c-percent_encoding]⮳{{hi:percent-encoding}} handles URL encoding and decoding.

{{#example percent-encoding}}

## JSON

[`serde_json`][c-serde_json]⮳{{hi:serde_json}}: A very popular crate for JSON serialization and deserialization. [`serde`][c-serde]⮳{{hi:serde}} is used for the underlying serialization framework.

[[json | JSON]]

## TOML (Tom's Obvious, Minimal Language)

[`toml`][c-toml]⮳{{hi:toml}} for working with TOML files.

[[toml | TOML]]

## YAML (YAML Ain't Markup Language)

[`serde_yml`][c-serde_yml]⮳{{hi:serde_yml}} for YAML serialization and deserialization. Uses [`serde`][c-serde]⮳{{hi:serde}}.

[[yaml | YAML]]

## BSON (Binary JSON)

[`bson`][c-bson]⮳{{hi:bson}} for working with BSON, a binary representation of JSON-like documents.

## MessagePack

[`rmp-serde`][c-rmp_serde]⮳{{hi:rmp-serde}} for MessagePack serialization and deserialization.

[[_binary_encoders |  Binary Encoders]]

## CBOR (Concise Binary Object Representation)

[`serde_cbor`][c-serde_cbor]⮳{{hi:serde_cbor}} for CBOR encoding and decoding.

[[_binary_encoders |  Binary Encoders]]

## XML

[`quick-xml`][c-quick_xml]⮳{{hi:quick-xml}} : A fast XML parser.
[`serde_xml_rs`][c-serde_xml_rs]⮳{{hi:serde_xml_rs}}: For serializing and deserializing XML.

[[xml | XML]]

## Binary Data (General)

[`byteorder`][c-byteorder]⮳{{hi:byteorder}}: For reading and writing binary data in different endianness.
[`bincode`][c-bincode]⮳{{hi:bincode}}: A crate for efficiently serializing and deserializing data in a compact binary format.

[[_binary_encoders |  Binary Encoders]]

## Data Serialization Framework (Used by many of the above)

[`serde`][c-serde]⮳{{hi:serde}}: The powerful and widely used serialization framework in Rust. Many of the encoding crates above rely on serde.

For general character encoding, encoding is a good choice. For JSON, [`serde_json`][c-serde_json]⮳{{hi:serde_json}} is the standard. For other formats, look for crates that support them specifically. [`serde`][c-serde]⮳{{hi:serde}} is the underlying engine for many of these, providing a consistent way to work with serialization.

[[complex_encoding | Complex Encoding]]

</div>
