# Compression

Algorithms for making data smaller.

## `tar`

{{#include tar.incl.md}}

## `flate2`

{{#include flate2.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO P1 also review zip crate

The `zip` crate allows you to create, open, and manipulate ZIP files, including adding, extracting, and deleting files and directories within the archive. The crate supports various features of the ZIP format, such as compression (using different algorithms), encryption, and metadata handling. It offers both streaming and buffered interfaces. The zip crate is commonly used for tasks like archiving files, distributing software, and handling data compression in applications.

`async-zip` is a Rust crate providing asynchronous support for reading and writing ZIP archives.  Building upon the foundation of the zip crate, async-zip leverages asynchronous programming paradigms (using async/await) to enable non-blocking operations on ZIP files. This is particularly beneficial in I/O-bound contexts, such as network applications or when working with large archives, as it allows other tasks to proceed while ZIP operations are in progress.  It integrates seamlessly with the tokio runtime (and other async runtimes). Like its synchronous counterpart, async-zip supports various ZIP features, including compression, encryption, and metadata handling, but with the added advantage of non-blocking execution.

General Purpose: flate2 (zlib, gzip, deflate), miniz_oxide (zlib), zstd (Zstandard), lz4
Specialized: brotli (Brotli), snap (Snappy)
Archiving: tar, zip
Data Serialization (Often used with compression): serde (not a compression crate, but crucial for preparing data for compression)

</div>
