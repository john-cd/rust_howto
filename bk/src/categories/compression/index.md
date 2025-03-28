# Compression

Algorithms for making data smaller.

| Purpose | Rust Crate(s) |
|---|---|
| General | [`flate2`][c-flate2]⮳{{hi:flate2}} ([`zlib`][c-zlib]⮳{{hi:zlib}}, [`gzip`][c-gzip]⮳{{hi:gzip}}, [`deflate`][c-deflate]⮳{{hi:deflate}}), [`miniz_oxide`][c-miniz_oxide]⮳{{hi:miniz_oxide}} (zlib), [`zstd`][c-zstd]⮳{{hi:zstd}} (Zstandard), [`lz4`][c-lz4]⮳{{hi:lz4}} |
| Specialized | [`brotli`][c-brotli]⮳{{hi:brotli}} (Brotli), [`snap`][c-snap]⮳{{hi:snap}} (Snappy) |
| Archiving | [`tar`][c-tar]⮳{{hi:tar}}, [`zip`][c-zip]⮳{{hi:zip}} |

## `tar`

{{#include tar.incl.md}}

## `flate2`

{{#include flate2.incl.md}}

## `zip` and `async-zip`

The [`zip`][c-zip]⮳{{hi:zip}} crate allows you to create, open, and manipulate ZIP files, including adding, extracting, and deleting files and directories within the archive. The crate supports various features of the ZIP format, such as compression (using different algorithms), encryption, and metadata handling. It offers both streaming and buffered interfaces. The zip crate is commonly used for tasks like archiving files, distributing software, and handling data compression in applications.

[`async-zip`][c-async_zip]⮳{{hi:async-zip}} is a Rust crate providing asynchronous support for reading and writing ZIP archives. Building upon the foundation of the zip crate, async-zip leverages asynchronous programming paradigms (using async/await) to enable non-blocking operations on ZIP files. This is particularly beneficial in I/O-bound contexts, such as network applications or when working with large archives, as it allows other tasks to proceed while ZIP operations are in progress. It integrates with the [`tokio`][c-tokio]⮳{{hi:tokio}} runtime (and other async runtimes). Like its synchronous counterpart, [`async-zip`][c-async_zip]⮳{{hi:async-zip}} supports various ZIP features, including compression, encryption, and metadata handling, but with the added advantage of non-blocking execution.

## Related Topics

- Data Serialization is often used with compression: see [[complex_encoding | Complex Encoding]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/1184)
</div>
