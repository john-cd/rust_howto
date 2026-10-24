<!-- TODO Reviewed ??-??-2026 -->

# Compression and Decompression

{{#include compression.incl.md}}

## Compress or Decompress Data with `flate2` {#compress-decompress}

[![flate2][c~flate2~docs~badge]][c~flate2~docs] [![flate2~crates.io][c~flate2~crates.io~badge]][c~flate2~crates.io] [![flate2~repo][c~flate2~repo~badge]][c~flate2~repo] [![flate2~lib.rs][c~flate2~lib.rs~badge]][c~flate2~lib.rs]{{hi:flate2}}{{hi:Encoding}}{{hi:Gzip}}{{hi:Zlib}}{{hi:Deflate}}{{hi:Zlib-ng}} [![cat~api-bindings][cat~api-bindings~badge]][cat~api-bindings]{{hi:API bindings}} [![cat~compression][cat~compression~badge]][cat~compression]{{hi:Compression}}

[`flate2`][c~flate2~docs]↗{{hi:flate2}} provides DEFLATE compression and decompression, exposed as Read / BufRead / Write streams. It supports zlib, gzip, and raw deflate streams.

[`flate2`][c~flate2~docs]↗{{hi:flate2}} uses a pure-Rust implementation by default. Use feature flags to opt in to system [`zlib`][zlib]↗{{hi:zlib}}.

Its `Read`, `BufRead`, and `Write` adapters compose with files, sockets, and in-memory buffers. Select the matching encoder and decoder for the format expected by the other side; gzip and zlib add format metadata around DEFLATE data.

The file example below uses unique temporary files and removes them automatically when they are dropped. It is marked `noplayground` because the browser playground cannot access the local filesystem.

```rust,editable,noplayground
{{#include ../../../crates/cats/compression/examples/compression/flate2.rs:example}}
```

## Compress and Decompress a File with Gzip {#gzip-file}

[![flate2][c~flate2~docs~badge]][c~flate2~docs] [![flate2~crates.io][c~flate2~crates.io~badge]][c~flate2~crates.io] [![flate2~repo][c~flate2~repo~badge]][c~flate2~repo] [![flate2~lib.rs][c~flate2~lib.rs~badge]][c~flate2~lib.rs]{{hi:flate2}}{{hi:Gzip}}{{hi:File I/O}} [![cat~compression][cat~compression~badge]][cat~compression]{{hi:Compression}}

Wrap a reader in `GzDecoder` to restore a gzip stream, or a writer in `GzEncoder` to create one. The example copies through temporary files and checks that the restored bytes match the original.

```rust,editable,noplayground
{{#include ../../../crates/cats/compression/examples/compression/flate2.rs:gzip-file}}
```

## Compress and Decompress Files Asynchronously with `async-compression` {#async-gzip-file}

[![async-compression][c~async-compression~docs~badge]][c~async-compression~docs] [![tokio][c~tokio~docs~badge]][c~tokio~docs]{{hi:Tokio}}{{hi:Asynchronous}} [![cat~compression][cat~compression~badge]][cat~compression]{{hi:Compression}} [![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}}

[`async-compression`][c~async-compression~docs]↗ adapts codecs such as gzip to asynchronous I/O traits. This example uses its Tokio adapter to copy a file into a gzip stream and then decompress it.

```rust,editable,noplayground
{{#include ../../../crates/cats/compression/examples/compression/async_compression.rs:example}}
```

## Create a ZIP Archive with `zip` {#zip}

[![zip][c~zip~docs~badge]][c~zip~docs] [![zip~crates.io][c~zip~crates.io~badge]][c~zip~crates.io] [![zip~repo][c~zip~repo~badge]][c~zip~repo] [![zip~lib.rs][c~zip~lib.rs~badge]][c~zip~lib.rs]{{hi:zip}}{{hi:Compression}}{{hi:zip}}{{hi:Archive}}

[`zip`][c~zip~docs]↗ creates ZIP archives with named entries and per-entry options. Use it when synchronous file or archive I/O fits the surrounding program; choose the compression method and enabled features to match the archive you need to produce.

```rust,editable,noplayground
{{#include ../../../crates/cats/compression/examples/compression/zip.rs:example}}
```

## Write and Read a ZIP Archive with `async_zip` {#async-zip}

[![async_zip][c~async_zip~docs~badge]][c~async_zip~docs] [![async_zip~crates.io][c~async_zip~crates.io~badge]][c~async_zip~crates.io] [![async_zip~repo][c~async_zip~repo~badge]][c~async_zip~repo] [![async_zip~lib.rs][c~async_zip~lib.rs~badge]][c~async_zip~lib.rs]{{hi:async_zip}}{{hi:Async}}{{hi:Zip}}{{hi:Archive}} [![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}} [![cat~compression][cat~compression~badge]][cat~compression]{{hi:Compression}}

Use `async_zip` when archive reads or writes need to participate in an async task. The example writes an entry, reads the archive back, and verifies its contents using Tokio and the crate's compatibility adapter.

```rust,editable,noplayground
{{#include ../../../crates/cats/compression/examples/compression/async_zip.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[filesystem | Filesystem]].
- [I/O streams][p~streams].
- [Error handling][p~error-handling].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
