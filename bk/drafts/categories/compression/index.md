<!-- TODO Reviewed ??-??-2026 -->

# Compression

Compression encodes a byte stream in a smaller representation; decompression requires a compatible codec and format. An archive groups named files and metadata. These operations are separate: a `.tar.gz` file is a tar archive compressed with gzip, while a ZIP file packages entries in its own archive format.

Choose a format that matches the files or protocol you need to read and write. Then consider whether the API should stream data, whether it should use asynchronous I/O, and how its CPU and size trade-off behaves on representative input. The recipes here use lossless compression.

| Task | Start here |
|---|---|
| Compress or decompress a byte stream using gzip, zlib, or raw DEFLATE | [`flate2` stream examples][ex~compression~compress-decompress] |
| Compress or decompress a stream with asynchronous I/O | [`async-compression` recipe][ex~compression~async-gzip-file] |
| Create a ZIP archive synchronously | [`zip` recipe][ex~compression~zip] |
| Write and read a ZIP archive asynchronously | [`async_zip` recipe][ex~compression~async-zip] |
| Pack or extract a directory as a gzip-compressed tarball | [Create a tarball][ex~compression~compress-a-directory-into-a-tarball] or [extract one][ex~compression~decompress-a-tarball] |

Other codec libraries in the Rust ecosystem include [`miniz_oxide`][c~miniz_oxide~docs]↗{{hi:miniz_oxide}}, [`zstd`][c~zstd~docs]↗{{hi:zstd}}, [`lz4`][c~lz4~docs]↗{{hi:lz4}}, [`brotli`][c~brotli~docs]↗{{hi:brotli}}, and [`snap`][c~snap~docs]↗{{hi:snap}}. Check the format and feature support for your use case before selecting one.

## Compress and Decompress Streams

The stream recipes show synchronous and asynchronous compression APIs:

{{#include compression.incl.md}}

## Archive Files and Directories

Tar archives can be combined with a compression codec such as gzip. ZIP archives package named entries and support per-entry compression:

{{#include tar.incl.md}}

## Related Topics

- [I/O streams][p~streams] let codecs process data incrementally.
- [Error handling][p~error-handling] matters when files or compressed input are incomplete or invalid.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
