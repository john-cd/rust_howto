# Compression and Decompression

{{#include compression.incl.md}}

## Compress or Decompress Data with `flate2` {#compress-decompress}

[![flate2][c-flate2-badge]][c-flate2] [![flate2-crates.io][c-flate2-crates.io-badge]][c-flate2-crates.io] [![flate2-github][c-flate2-github-badge]][c-flate2-github] [![flate2-lib.rs][c-flate2-lib.rs-badge]][c-flate2-lib.rs]{{hi:flate2}}{{hi:Encoding}}{{hi:Gzip}}{{hi:Zlib}}{{hi:Deflate}}{{hi:Zlib-ng}} [![cat-api-bindings][cat-api-bindings-badge]][cat-api-bindings]{{hi:API bindings}} [![cat-compression][cat-compression-badge]][cat-compression]{{hi:Compression}}

[`flate2`][c-flate2]⮳{{hi:flate2}} provides DEFLATE compression and decompression, exposed as Read / BufRead / Write streams. It supports zlib, gzip, and raw deflate streams.

[`flate2`][c-flate2]⮳{{hi:flate2}} uses a pure-Rust implementation by default. Use feature flags to opt in to system [`zlib`][c-zlib]⮳{{hi:zlib}}.

It can be used with various I/O [streams][p-streams], making it versatile for different use cases, such as file [compression][p-compression], network protocols, and in-memory data manipulation. It also offers features like checksumming and [error handling][p-error-handling], making it suitable for production environments.

```rust,editable,noplayground
{{#include ../../../crates/cats/compression/tests/compression/flate2.rs:example}}
```

The following demonstrates asynchronous compression:

```rust,editable,noplayground
{{#include ../../../crates/cats/compression/tests/compression/async_compression.rs:example}}
```

## Compress or Decompress Data with `zip` and `async-zip` {#zip}

[![zip][c-zip-badge]][c-zip] [![zip-crates.io][c-zip-crates.io-badge]][c-zip-crates.io] [![zip-github][c-zip-github-badge]][c-zip-github] [![zip-lib.rs][c-zip-lib.rs-badge]][c-zip-lib.rs]{{hi:zip}}{{hi:Compression}}{{hi:zip}}{{hi:Archive}}

[![async_zip][c-async_zip-badge]][c-async_zip] [![async_zip-crates.io][c-async_zip-crates.io-badge]][c-async_zip-crates.io] [![async_zip-github][c-async_zip-github-badge]][c-async_zip-github] [![async_zip-lib.rs][c-async_zip-lib.rs-badge]][c-async_zip-lib.rs]{{hi:async_zip}}{{hi:Async}}{{hi:Zip}}{{hi:Archive}}{{hi:Tokio}} [![cat-asynchronous][cat-asynchronous-badge]][cat-asynchronous]{{hi:Asynchronous}} [![cat-compression][cat-compression-badge]][cat-compression]{{hi:Compression}}

The [`zip`][c-zip]⮳{{hi:zip}} crate allows you to create, open, and manipulate ZIP files, including adding, extracting, and deleting files and directories within the archive. The crate supports various features of the ZIP format, such as compression (using different algorithms), encryption, and metadata handling. It offers both streaming and buffered interfaces. The `zip` crate is commonly used for tasks like archiving files, distributing software, and handling data compression in applications.

[`async-zip`][c-async_zip]⮳{{hi:async-zip}} is a Rust crate providing asynchronous support for reading and writing ZIP archives. Building upon the foundation of the `zip` crate, async-zip leverages asynchronous programming paradigms (using async/await) to enable non-blocking operations on ZIP files. This is particularly beneficial in I/O-bound contexts, such as network applications or when working with large archives, as it allows other tasks to proceed while ZIP operations are in progress. It integrates with the [`tokio`][c-tokio]⮳{{hi:tokio}} runtime (and other async runtimes). Like its synchronous counterpart, [`async-zip`][c-async_zip]⮳{{hi:async-zip}} supports various ZIP features, including compression, encryption, and metadata handling, but with the added advantage of non-blocking execution.

```rust,editable,noplayground
{{#include ../../../crates/cats/compression/tests/compression/zip.rs:example}}
```

## Related Topics {#skip}

- [[filesystem | Filesystem]].
- [[compression | Compression]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write NOW](https://github.com/john-cd/rust_howto/issues/1062)
</div>
