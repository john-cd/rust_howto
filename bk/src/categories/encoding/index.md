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

`encoding`: A comprehensive crate for working with various character encodings (UTF-8, UTF-16, Latin-1, etc.). A good general-purpose choice.
Base64:

`base64`: A widely used crate for Base64 encoding and decoding.

## URL Encoding

`percent-encoding`: Handles URL encoding and decoding.

## JSON

`serde_json`: A very popular crate for JSON serialization and deserialization. serde is used for the underlying serialization framework.

## TOML (Tom's Obvious, Minimal Language)

`toml`: For working with TOML files.

## YAML (YAML Ain't Markup Language)

`serde_yaml`: For YAML serialization and deserialization. Uses serde.

## BSON (Binary JSON)

`bson`: For working with BSON, a binary representation of JSON-like documents.

## MessagePack

`rmp-serde`: For MessagePack serialization and deserialization.

## CBOR (Concise Binary Object Representation)

`serde_cbor`: For CBOR encoding and decoding.

## XML

`quick-xml`: A fast XML parser.
`serde_xml_rs`: For serializing and deserializing XML.

## Binary Data (General)

`byteorder`: For reading and writing binary data in different endianness.
`bincode`: A crate for efficiently serializing and deserializing data in a compact binary format.

## Data Serialization Framework (Used by many of the above)

`serde`: The powerful and widely used serialization framework in Rust. Many of the encoding crates above rely on serde.

It's important to choose the right encoding crate for the job. For general character encoding, encoding is a good choice. For JSON, serde_json is the standard. For other formats, look for crates that support them specifically. serde is the underlying engine for many of these, providing a consistent way to work with serialization.
</div>
