# Compression

This section covers algorithms for making data smaller.

| Purpose | Rust Crate(s) |
|---|---|
| General Compression | [`flate2`][c~flate2~docs]↗{{hi:flate2}} ([`zlib`][c~zlib~docs]↗{{hi:zlib}}, [`gzip`][c~gzip~docs]↗{{hi:gzip}}, [`deflate`][c~deflate~docs]↗{{hi:deflate}}), [`miniz_oxide`][c~miniz_oxide~docs]↗{{hi:miniz_oxide}} (zlib), [`zstd`][c~zstd~docs]↗{{hi:zstd}} (Zstandard), [`lz4`][c~lz4~docs]↗{{hi:lz4}} |
| Specialized Compression | [`brotli`][c~brotli~docs]↗{{hi:brotli}} (Brotli), [`snap`][c~snap~docs]↗{{hi:snap}} (Snappy) |
| Archiving | [`tar`][c~tar~docs]↗{{hi:tar}}, [`zip`][c~zip~docs]↗{{hi:zip}} |

## Working with Tarballs

{{#include tar.incl.md}}

## Compression and Decompression

{{#include compression.incl.md}}

## Related Topics

Data Serialization is often used with compression: see [[complex_encoding | Complex Encoding]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/1184)
</div>
