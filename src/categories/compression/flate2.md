# Deflate

## Compress then decompress {#compress-decompress}

[![flate2][c-flate2-badge]][c-flate2]{{hi:flate2}} [![cat-compression][cat-compression-badge]][cat-compression]{{hi:Compression}}

`flate2` provides DEFLATE compression and decompression, exposed as Read/BufRead/Write streams. `flate2` uses a pure-Rust implementation by default. Use feature flags to opt in to system `zlib`.

Supported formats:

- deflate
- zlib
- gzip

Compress, then decompress data:

```rust,editable,noplayground
{{#include ../../../crates/ex/cats/compression/tests/gzip/flate2.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO P1 write; add to SUMMARY
</div>
