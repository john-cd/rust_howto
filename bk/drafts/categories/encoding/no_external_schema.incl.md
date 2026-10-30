<!-- TODO Reviewed ??-??-2026 -->

These approaches use Rust types (usually described with [`serde`][ex~encoding~serde]) as the mapping for encoding and decoding values. They do not require a separate schema file or generated types. That does not make every payload self-describing: formats differ in how much type information and field names they put on the wire, and a concrete Rust type still supplies application-level meaning.

| Format      | Crate                                | Self-describing payload?            | `no_std`?                                         | Typical use                                                             |
| ----------- | ------------------------------------ | ----------------------------------- | ------------------------------------------------- | ----------------------------------------------------------------------- |
| Postcard    | [`postcard`][ex~encoding~postcard]   | No                                  | Yes                                               | Compact messages for embedded and other constrained systems             |
| Bincode     | [`bincode`][ex~encoding~bincode]     | No                                  | Yes, with default features disabled               | Compact Rust-to-Rust storage or transport with a shared type definition |
| MessagePack | [`rmp-serde`][ex~encoding~rmp-serde] | Yes, when decoded as generic values | No; it uses `std`                                 | Binary interchange; named maps can preserve struct field names          |
| CBOR        | [`ciborium`][ex~encoding~ciborium]   | Yes                                 | Yes, with default features disabled; uses `alloc` | Standardized interchange, including constrained systems                 |

Here, _self-describing_ means the format carries enough type information to decode a value without first choosing a Rust struct. It does not carry application semantics or replace versioned type definitions. MessagePack structs are encoded as positional arrays by `rmp_serde::to_vec`; use `rmp_serde::to_vec_named` to encode field names as a map.

These formats contrast with [schema-first binary encoders][p~binary-encoders], which define a separate contract such as a `.proto`, `.fbs`, or `.capnp` file and generate types from it.

| Recipe                               | Crates               | Categories                        |
| ------------------------------------ | -------------------- | --------------------------------- |
| [`bincode`][ex~encoding~bincode]     | {{#crate bincode}}   | {{#categories encoding=Encoding}} |
| [`ciborium`][ex~encoding~ciborium]   | {{#crate ciborium}}  | {{#categories encoding=Encoding}} |
| [`postcard`][ex~encoding~postcard]   | {{#crate postcard}}  | {{#categories encoding=Encoding}} |
| [`rmp-serde`][ex~encoding~rmp-serde] | {{#crate rmp-serde}} | {{#categories encoding=Encoding}} |
| [`rkyv`][ex~encoding~rkyv]           | {{#crate rkyv}}      | {{#categories encoding=Encoding}} |
