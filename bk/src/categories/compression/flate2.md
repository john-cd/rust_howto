# Deflate

## Compress then decompress {#compress-decompress}

[![flate2][c-flate2-badge]][c-flate2] [![flate2-crates.io][c-flate2-crates.io-badge]][c-flate2-crates.io] [![flate2-github][c-flate2-github-badge]][c-flate2-github] [![flate2-lib.rs][c-flate2-lib.rs-badge]][c-flate2-lib.rs]{{hi:flate2}}{{hi:Encoding}}{{hi:Gzip}}{{hi:Zlib}}{{hi:Deflate}}{{hi:Zlib-ng}} [![cat-api-bindings][cat-api-bindings-badge]][cat-api-bindings]{{hi:API bindings}} [![cat-compression][cat-compression-badge]][cat-compression]{{hi:Compression}}

[`flate2`][c-flate2]⮳{{hi:flate2}} provides DEFLATE compression and decompression, exposed as Read/BufRead/Write streams. [`flate2`][c-flate2]⮳{{hi:flate2}} uses a pure-Rust implementation by default. Use feature flags to opt in to system [`zlib`][c-zlib]⮳{{hi:zlib}}. Supports zlib, gzip, and raw deflate streams.

The [`flate2`][c-flate2]⮳{{hi:flate2}} crate in Rust provides bindings to the zlib, zlib-ng, and minizlib compression libraries. It offers a comprehensive set of functionalities for working with various compression formats, including gzip, deflate, and zlib. [`flate2`][c-flate2]⮳{{hi:flate2}} provides both high-level and low-level interfaces. It supports both compression and decompression operations and can be used with various I/O [streams][p-streams], making it versatile for different use cases, such as file [compression][p-compression], network protocols, and in-memory data manipulation. The crate leverages the underlying C libraries for optimal performance. It also offers features like checksumming and [error handling][p-error-handling], making it suitable for production environments.

Compress, then decompress data:

```rust,editable,noplayground
{{#include ../../../crates/cats/compression/tests/gzip/flate2.rs:example}}
```

## Related Topics {#skip}

- [[filesystem | Filesystem]].
- [[compression | Compression]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/1062)
</div>
