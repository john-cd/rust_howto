## Decompress a tarball

[![flate2-badge]][flate2] [![tar-badge]][tar] [![cat-compression-badge]][cat-compression]

Decompress ([`GzDecoder`]) and
extract ([`Archive::unpack`]) all files from a compressed tarball
named `archive.tar.gz` located in the current working directory
to the same location.

```rust,editable,no_run
{#include ../../../deps/examples/tar-decompress.rs}
```

[`Archive::unpack`]: https://docs.rs/tar/*/tar/struct.Archive.html#method.unpack
[`GzDecoder`]: https://docs.rs/flate2/*/flate2/read/struct.GzDecoder.html
