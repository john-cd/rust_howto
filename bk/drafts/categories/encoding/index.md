# Encoding

[![cat~encoding][cat~encoding~badge]][cat~encoding]{{hi:Encoding}}

Encoding and/or decoding{{hi:Decoding}} data from one data format to another.

For general character encoding, [`encoding`][c~encoding~docs]↗{{hi:encoding}} is a good choice. For JSON, [`serde_json`][c~serde_json~docs]↗{{hi:serde_json}} is the standard. For other formats, look for crates that support them specifically. [`serde`][c~serde~docs]↗{{hi:serde}} is the underlying engine for many of these, providing a consistent way to work with serialization.

## Character Encoding

- [`encoding`][c~encoding~docs]↗{{hi:encoding}}: A comprehensive crate for working with various character encodings (UTF-8, UTF-16, Latin-1, etc.). A good general-purpose choice.
- [`base64`][c~base64~docs]↗{{hi:base64}}: A widely used crate for Base64 encoding and decoding.

{{#include string_encoding.incl.md}}

## Serialization / Deserialization (Serde)

- [`serde`][c~serde~docs]↗{{hi:serde}}: The powerful and widely used serialization framework in Rust. Many of the encoding crates above rely on serde. See [[complex_encoding | Complex Encoding]].

{{#include serde.incl.md}}

## Structured Data (Complex Encoding)

{{#include complex_encoding.incl.md}}

## CSV Processing

{{#include csv.incl.md}}

## JSON Serialization

- [`serde_json`][c~serde_json~docs]↗{{hi:serde_json}}: A very popular crate for JSON serialization and deserialization.
See [[json | JSON]].

## BSON (Binary JSON) Serialization

Use [`bson`][c~bson~docs]↗{{hi:bson}} for working with BSON, a binary representation of JSON-like documents.

## TOML (Tom's Obvious, Minimal Language) Serialization

Consider:

- [`toml`][c~toml~docs]↗{{hi:toml}} for working with TOML files - see [[toml | TOML]].

## YAML (YAML Ain't Markup Language) Serialization

Consider:

- [`serde_yml`][c~serde_yml~docs]↗{{hi:serde_yml}} for YAML serialization and deserialization. Uses [`serde`][c~serde~docs]↗{{hi:serde}}.
See [[yaml | YAML]].

## XML Serialization

Use:

- [`quick-xml`][c~quick-xml~docs]↗{{hi:quick-xml}} : A fast XML parser.
- [`serde-xml-rs`][c~serde-xml-rs~docs]↗{{hi:serde-xml-rs}}: For serializing and deserializing XML.
- [`serde`][c~serde~docs]↗{{hi:serde}}.

See [[xml | XML]].

## Binary Encoders

{{#include _binary_encoders.incl.md}}

## Binary Data Serialization (General)

[`byteorder`][c~byteorder~docs]↗{{hi:byteorder}}: For reading and writing binary data in different endianness.
[`bincode`][c~bincode~docs]↗{{hi:bincode}}: A crate for efficiently serializing and deserializing data in a compact binary format.

See [[_binary_encoders |  Binary Encoders]].

## Binary Encoders Without External Schemas

{{#include no_external_schema.incl.md}}

## MessagePack Serialization

Consider using [`rmp-serde`][c~rmp-serde~docs]↗{{hi:rmp-serde}} for MessagePack serialization and deserialization.
See [[_binary_encoders |  Binary Encoders]].

## CBOR (Concise Binary Object Representation) Serialization

Use [`serde_cbor`][c~serde_cbor~docs]↗{{hi:serde_cbor}} for CBOR encoding and decoding.
See [[_binary_encoders |  Binary Encoders]].

## Protocol Buffers

Review [`protobuf`][c~protobuf~docs]↗{{hi:protobuf}}, [`prost`][c~prost~docs]↗{{hi:prost}}.
See [[_binary_encoders | Binary Encoders]].

### gRPC

Review [`tonic`][c~tonic~docs]↗{{hi:tonic}}, [`grpc`][c~grpc~docs]↗{{hi:grpc}}. See [[_grpc | gRPC]].

## Typecasts

{{#include typecasts.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[review](https://github.com/john-cd/rust_howto/issues/929)

- [encoding_rs — web char encoding library][c~encoding_rs~lib.rs]

</div>
