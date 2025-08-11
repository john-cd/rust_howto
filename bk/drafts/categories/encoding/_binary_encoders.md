# Binary Encoders

{{#include binary_encoders.incl.md}}

## `bincode` {#bincode}

[![bincode][c~bincode~docs~badge]][c~bincode~docs] [![bincode~crates.io][c~bincode~crates.io~badge]][c~bincode~crates.io] [![bincode~github][c~bincode~github~badge]][c~bincode~github] [![bincode~lib.rs][c~bincode~lib.rs~badge]][c~bincode~lib.rs]{{hi:bincode}}{{hi:Deserialize}}{{hi:Serialize}}{{hi:Binary}}{{hi:Encode}}{{hi:Decode}} [![cat~encoding][cat~encoding~badge]][cat~encoding]{{hi:Encoding}} [![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}}

[`bincode`][c~bincode~docs]↗{{hi:bincode}} is a binary serialization / deserialization strategy for transforming structs into bytes and vice versa.

```rust,editable
{{#include ../../../crates/cats/encoding/examples/binary_encoders/bincode.rs:example}}
```

## ProtoBuf {#skip1}

### `prost` {#prost}

[![prost][c~prost~docs~badge]][c~prost~docs] [![prost~crates.io][c~prost~crates.io~badge]][c~prost~crates.io] [![prost~github][c~prost~github~badge]][c~prost~github] [![prost~lib.rs][c~prost~lib.rs~badge]][c~prost~lib.rs]{{hi:prost}}{{hi:Protobuf}}{{hi:Serialization}} [![cat~encoding][cat~encoding~badge]][cat~encoding]{{hi:Encoding}}

[`prost`][c~prost~docs]↗{{hi:prost}} is a Protocol Buffers implementation for the Rust Language.

In you `main.rs` file:

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

### `protobuf` {#protobuf}

[![protobuf][c~protobuf~docs~badge]][c~protobuf~docs] [![protobuf~crates.io][c~protobuf~crates.io~badge]][c~protobuf~crates.io] [![protobuf~github][c~protobuf~github~badge]][c~protobuf~github] [![protobuf~lib.rs][c~protobuf~lib.rs~badge]][c~protobuf~lib.rs]{{hi:protobuf}}

[`protobuf`][c~protobuf~docs]↗{{hi:protobuf}} is a Rust implementation of Google protocol buffers.

```rust,editable
{{#include ../../../crates/cats/encoding/examples/binary_encoders/protobuf.rs:example}}
```

## MessagePack with `rmp-serde` {#rmp-serde}

[![rmp-serde][c~rmp-serde~docs~badge]][c~rmp-serde~docs] [![rmp-serde~crates.io][c~rmp-serde~crates.io~badge]][c~rmp-serde~crates.io] [![rmp-serde~github][c~rmp-serde~github~badge]][c~rmp-serde~github] [![rmp-serde~lib.rs][c~rmp-serde~lib.rs~badge]][c~rmp-serde~lib.rs]{{hi:rmp-serde}}{{hi:MessagePack}}{{hi:MessagePack}}{{hi:Serialization}}{{hi:Serde}} [![cat~encoding][cat~encoding~badge]][cat~encoding]{{hi:Encoding}}

[`rmp-serde`][c~rmp-serde~docs]↗{{hi:rmp-serde}} connects the `MessagePack` library with [`serde`][c~serde~docs]↗{{hi:serde}}, providing the ability to easily serialize and deserialize Rust built-in types, types from the standard library, and custom data structures.

```rust,editable
{{#include ../../../crates/cats/encoding/examples/binary_encoders/rmp-serde.rs:example}}
```

## CBOR with `ciborium` {#ciborium}

[![ciborium][c~ciborium~docs~badge]][c~ciborium~docs] [![ciborium~crates.io][c~ciborium~crates.io~badge]][c~ciborium~crates.io] [![ciborium~github][c~ciborium~github~badge]][c~ciborium~github] [![ciborium~lib.rs][c~ciborium~lib.rs~badge]][c~ciborium~lib.rs]{{hi:ciborium}}{{hi:CBOR}}{{hi:Serde}} [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}} [![cat~embedded][cat~embedded~badge]][cat~embedded]{{hi:Embedded development}} [![cat~encoding][cat~encoding~badge]][cat~encoding]{{hi:Encoding}} [![cat~parsing][cat~parsing~badge]][cat~parsing]{{hi:Parsing tools}}

Concise Binary Object Representation is a binary data serialization format loosely based on [JSON][p~json]. [`ciborium`][c~ciborium~docs]↗{{hi:ciborium}} is a [`serde`][c~serde~docs]↗{{hi:serde}} implementation of CBOR using ciborium-basic.

```rust,editable
{{#include ../../../crates/cats/encoding/examples/binary_encoders/ciborium.rs:example}}
```

## Flatbuffers {#flatbuffers}

[![flatbuffers~website][c~flatbuffers~website~badge]][c~flatbuffers~website] [![flatbuffers][c~flatbuffers~docs~badge]][c~flatbuffers~docs] [![flatbuffers~crates.io][c~flatbuffers~crates.io~badge]][c~flatbuffers~crates.io] [![flatbuffers~github][c~flatbuffers~github~badge]][c~flatbuffers~github] [![flatbuffers~lib.rs][c~flatbuffers~lib.rs~badge]][c~flatbuffers~lib.rs]{{hi:flatbuffers}}{{hi:flatbuffers}}{{hi:Serialization}}{{hi:Zero-copy}} [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}} [![cat~encoding][cat~encoding~badge]][cat~encoding]{{hi:Encoding}} [![cat~memory-management][cat~memory-management~badge]][cat~memory-management]{{hi:Memory management}}

[`flatbuffers`][c~flatbuffers~docs]↗{{hi:flatbuffers}} is the official FlatBuffers Rust runtime library. FlatBuffers is a free software library implementing a serialization format similar to Protocol Buffers, Thrift, Apache Avro, SBE, and Cap'n Proto, open-sourced by Google. It supports "zero-copy" deserialization, so that accessing the serialized data does not require first copying it into a separate part of memory.

```rust,editable
{{#include ../../../crates/cats/encoding/examples/binary_encoders/flatbuffers.rs:example}}
```

## Cap'n Proto {#capnp}

[![capnp][c~capnp~docs~badge]][c~capnp~docs] [![capnp~crates.io][c~capnp~crates.io~badge]][c~capnp~crates.io] [![capnp~github][c~capnp~github~badge]][c~capnp~github] [![capnp~lib.rs][c~capnp~lib.rs~badge]][c~capnp~lib.rs]{{hi:capnp}} [![capnpc][c~capnpc~docs~badge]][c~capnpc~docs] [![capnpc~crates.io][c~capnpc~crates.io~badge]][c~capnpc~crates.io] [![capnpc~github][c~capnpc~github~badge]][c~capnpc~github] [![capnpc~lib.rs][c~capnpc~lib.rs~badge]][c~capnpc~lib.rs]{{hi:capnpc}}{{hi:Encoding}}{{hi:Protocol}}{{hi:Serialization}}

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

### Key Features of Cap'n Proto {#skip2}

- Compact and efficient: Cap'n Proto is designed to be very space-efficient for both on-the-wire and in-memory representations.
- Fast: Cap'n Proto offers excellent [performance][p~performance], especially for serialization and deserialization.
- Language-agnostic: You can generate code in various [programming languages][p~programming-languages] from a single .capnp definition file.
- Schema evolution: Cap'n Proto supports schema evolution, allowing you to modify the structure of your data over time without breaking compatibility.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[binary_encoders: write; add anchors; add examples; add table; add to index / SUMMARY](https://github.com/john-cd/rust_howto/issues/349)

- [pot](https://docs.rs/crate/pot)
- [bytes][c~bytes~lib.rs]

</div>
