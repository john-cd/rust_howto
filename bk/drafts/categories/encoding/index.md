<!-- TODO Reviewed ??-??-2026 -->

# Encoding

[![cat~encoding][cat~encoding~badge]][cat~encoding]{{hi:Encoding}}

Encoding covers converting text between character sets and serializing structured values into bytes or formats such as JSON, CSV, and Protocol Buffers. Choose a format crate that matches the data and compatibility requirements; [`serde`][c~serde~docs]↗{{hi:serde}} provides a common data model for many Rust formats.

## Character Encoding

Convert text to and from byte representations, including Base64 and URL-safe encodings; use [`encoding_rs`][c~encoding_rs~docs]↗{{hi:encoding_rs}} when Web-compatible legacy character encodings are required. See [String Encoding](string_encoding.md).

## Serialization / Deserialization (Serde)

Use Serde to describe Rust data independently of a particular serialization format. See [Serde](serde.md).

## Structured Data (Complex Encoding)

Choose a structured format such as TOML, JSON, YAML, or XML when data needs named fields and a defined external representation. See [Complex Encoding](complex_encoding.md).

## CSV Processing

Read, validate, filter, transform, and write tabular text with the CSV format. See [CSV Processing](csv.md).

## Excel Spreadsheets

Read `.xlsx` files with [`calamine`][c~calamine~docs]↗{{hi:calamine}} and write them with [`rust_xlsxwriter`][c~rust_xlsxwriter~docs]↗{{hi:rust_xlsxwriter}}. See [Excel Spreadsheets](excel.md).

## JSON Serialization

Use [`serde_json`][c~serde_json~docs]↗{{hi:serde_json}} to serialize and deserialize JSON values. See [JSON](../parser-implementations/json.md).

## BSON (Binary JSON) Serialization

Use [`bson`][c~bson~docs]↗{{hi:bson}} for binary documents compatible with the BSON data model. See [Complex Encoding](complex_encoding.md).

## TOML (Tom's Obvious, Minimal Language) Serialization

Use [`toml`][c~toml~docs]↗{{hi:toml}} for a human-readable configuration format with table and array support. See [TOML](../parser-implementations/toml.md).

## YAML (YAML Ain't Markup Language) Serialization

Use [`yaml_serde`][c~yaml_serde~docs]↗{{hi:yaml_serde}}, the actively maintained Serde-compatible YAML crate, when YAML interoperability is required. See [YAML](../parser-implementations/yaml.md).

## XML Serialization

Use [`quick-xml`][c~quick-xml~docs]↗{{hi:quick-xml}} for XML parsing and Serde integration. See [XML](../parser-implementations/xml.md).

## Binary Encoders

Binary formats encode structured data compactly or according to an external schema. See [Binary Encoders](_binary_encoders.md).

## Binary Data Serialization (General)

For compact Rust data serialization, compare [`bincode`][c~bincode~docs]↗{{hi:bincode}} with the schema and compatibility needs of the application. See [Binary Encoders](_binary_encoders.md).

## Binary Encoders Without External Schemas

Formats such as Postcard and rkyv encode values without a separately defined schema; review their compatibility and validation model. See [Binary Encoders Without External Schemas](no_external_schema.md).

## MessagePack Serialization

Use [`rmp-serde`][c~rmp-serde~docs]↗{{hi:rmp-serde}} to encode Serde values in MessagePack. See [Binary Encoders](_binary_encoders.md).

## CBOR (Concise Binary Object Representation) Serialization

Use [`ciborium`][c~ciborium~docs]↗{{hi:ciborium}} to encode and decode CBOR through Serde; check wire-format requirements with other implementations. See [Binary Encoders](_binary_encoders.md).

## Protocol Buffers

Use [`prost`][c~prost~docs]↗{{hi:prost}} with a `.proto` schema and generated Rust types. See [Binary Encoders](_binary_encoders.md).

### gRPC

Combine Protocol Buffers with [`tonic`][c~tonic~docs]↗{{hi:tonic}} when building a gRPC service. See [gRPC](../web-programming_http-server/_grpc.md).

## Typecasts

For in-memory reinterpretation and byte-level views, check alignment, validity, and ownership requirements before using casts or helper crates. See [Typecasts](typecasts.md).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
