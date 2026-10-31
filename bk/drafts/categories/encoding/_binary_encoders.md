<!-- TODO Reviewed ??-??-2026 -->

# Binary Encoders

{{#include _binary_encoders.incl.md}}

## `bincode` {#bincode}

{{#badges bincode encoding=Encoding network-programming="Network programming"}}{{hi:Deserialize}}{{hi:Serialize}}{{hi:Binary}}{{hi:Encode}}{{hi:Decode}}

[`bincode`][c~bincode~docs]↗{{hi:bincode}} is a binary serialization / deserialization strategy for transforming structs into bytes and vice versa.

```rust,editable
{{#include ../../../crates/cats/encoding/examples/binary_encoders/bincode.rs:example}}
```

## Protocol Buffers {#protobuf .skip}

### `prost` {#prost}

{{#badges prost encoding=Encoding}}{{hi:Protobuf}}{{hi:Serialization}}

[`prost`][c~prost~docs]↗{{hi:prost}} is a Rust implementation of Protocol Buffers. This example uses `protox` and `prost-build` to generate Rust types from a `.proto` schema, then demonstrates a message round trip. The build script parses the schema without requiring an external `protoc` executable.

In the `main.rs` file:

```rust,editable
{{#include ../../../crates/cats/encoding/examples/binary_encoders/prost.rs:example}}
```

In `person.proto`:

```protobuf
{{#include ../../../crates/cats/encoding/examples/binary_encoders/person.proto}}
```

In `build.rs`:

```rust,editable
{{#include ../../../crates/cats/encoding/build.rs:prost}}
```

## CBOR with `ciborium` {#ciborium}

{{#badges ciborium data-structures="Data structures" no-std="No standard library" embedded="Embedded development" encoding=Encoding parsing="Parsing tools"}}{{hi:CBOR}}{{hi:Serde}}

Concise Binary Object Representation is a binary data serialization format loosely based on [JSON][p~json]. [`ciborium`][c~ciborium~docs]↗{{hi:ciborium}} is a [`serde`][c~serde~docs]↗{{hi:serde}} implementation of CBOR using ciborium-basic.

```rust,editable
{{#include ../../../crates/cats/encoding/examples/binary_encoders/ciborium.rs:example}}
```

## Flatbuffers {#flatbuffers}

[![flatbuffers~website][c~flatbuffers~website~badge]][c~flatbuffers~website] {{#badges flatbuffers}} {{hi:flatbuffers}} {{hi:Serialization}} {{hi:Zero-copy}} {{#categories data-structures="Data structures",encoding=Encoding,memory-management="Memory management"}}

[`flatbuffers`][c~flatbuffers~docs]↗{{hi:flatbuffers}} is the official FlatBuffers Rust runtime library. FlatBuffers is a free software library implementing a serialization format similar to Protocol Buffers, Thrift, Apache Avro, SBE, and Cap'n Proto, open-sourced by Google. It supports "zero-copy" deserialization, so that accessing the serialized data does not require first copying it into a separate part of memory.

From the repository root, compile the schema with `flatc --rust -o bk/crates/cats/encoding/examples/binary_encoders bk/crates/cats/encoding/examples/binary_encoders/my_game.fbs`. The generated Rust source is checked in beside the example so builds do not need `flatc`.

```rust,editable
{{#include ../../../crates/cats/encoding/examples/binary_encoders/flatbuffers.rs:example}}
```

In `my_game.fbs`:

```text
{{#include ../../../crates/cats/encoding/examples/binary_encoders/my_game.fbs}}
```

## Cap'n Proto {#capnp}

{{#badges capnp}} {{#badges capnpc}} {{hi:Encoding}} {{hi:Protocol}} {{hi:Serialization}}

[`capnp`][c~capnp~docs]↗{{hi:capnp}} is the runtime library for Cap'n Proto data encoding. [`capnpc`][c~capnpc~docs]↗{{hi:capnpc}} is used for Cap'n Proto code generation.

```rust,editable
{{#include ../../../crates/cats/encoding/examples/binary_encoders/capnp.rs:example}}
```

In `build.rs`:

```rust,editable
{{#include ../../../crates/cats/encoding/build.rs:capnp}}
```

In `foo.capnp`:

```txt
{{#include ../../../crates/cats/encoding/examples/binary_encoders/foo.capnp}}
```

### Key Features of Cap'n Proto {#key-features .skip}

- Compact and efficient: Cap'n Proto is designed to be very space-efficient for both on-the-wire and in-memory representations.
- Fast: Cap'n Proto offers excellent [performance][p~performance], especially for serialization and deserialization.
- Language-agnostic: You can generate code in various [programming languages][p~programming-languages] from a single `.capnp` definition file.
- Schema evolution: Cap'n Proto supports schema evolution, allowing us to modify the structure of your data over time without breaking compatibility.

## Related Topics {#related-topics .skip}

- [[complex_encoding | Structured Data]].
- [[serde | Serialization with `serde`]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[binary_encoders: write; add anchors; add examples; add table; add to index / SUMMARY](https://github.com/john-cd/rust_howto/issues/349)

- [pot][c~pot~docs]↗.
- [`protobuf`][c~protobuf~docs]↗: another Protocol Buffers implementation; its recipe was dropped in favor of `prost`.
- [bytes][c~bytes~lib.rs]↗.

</div>
