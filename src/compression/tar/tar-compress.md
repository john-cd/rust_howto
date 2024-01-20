## Compress a directory into tarball

[![flate2-badge]][flate2] [![tar-badge]][tar] [![cat-compression-badge]][cat-compression]

Compress `/var/log` directory into `archive.tar.gz`.

Creates a [`File`] wrapped in [`GzEncoder`]
and [`tar::Builder`]. </br>Adds contents of `/var/log` directory recursively into the archive
under `backup/logs`path with [`Builder::append_dir_all`].
[`GzEncoder`] is responsible for transparently compressing the
data prior to writing it into `archive.tar.gz`.

```rust,editable,no_run
{#include ../../../deps/examples/tar-compress.rs}
```

[`Builder::append_dir_all`]: https://docs.rs/tar/*/tar/struct.Builder.html#method.append_dir_all
[`File`]: https://doc.rust-lang.org/std/fs/struct.File.html
[`GzEncoder`]: https://docs.rs/flate2/*/flate2/write/struct.GzEncoder.html
[`tar::Builder`]: https://docs.rs/tar/*/tar/struct.Builder.html
