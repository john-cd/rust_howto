<!-- TODO Reviewed ??-??-2026 -->

Use `flate2` for common gzip, zlib, and raw DEFLATE streams. Add `async-compression` when those codecs need Tokio-based asynchronous I/O; use `zip` or `async_zip` when the task is to package named entries in an archive.

| Recipe                                                                                                  | Crates                       | Categories                                                        |
| ------------------------------------------------------------------------------------------------------- | ---------------------------- | ----------------------------------------------------------------- |
| [Compress or Decompress Data with `flate2`][ex~compression~compress-decompress]                         | {{#crate flate2}}            | {{#categories compression=Compression}}                           |
| [Compress and Decompress a File with Gzip][ex~compression~gzip-file]                                    | {{#crate flate2}}            | {{#categories compression=Compression}}                           |
| [Compress and Decompress Files Asynchronously with `async-compression`][ex~compression~async-gzip-file] | {{#crate async-compression}} | {{#categories compression=Compression asynchronous=Asynchronous}} |
| [Create a ZIP Archive with `zip`][ex~compression~zip]                                                   | {{#crate zip}}               | {{#categories compression=Compression}}                           |
| [Write and Read a ZIP Archive with `async_zip`][ex~compression~async-zip]                               | {{#crate async_zip}}         | {{#categories compression=Compression asynchronous=Asynchronous}} |
