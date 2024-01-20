# Working with Tarballs

## Decompress a tarball

[![flate2-badge]][flate2] [![tar-badge]][tar] [![cat-compression-badge]][cat-compression]

Decompress ([`GzDecoder`]) and extract ([`Archive::unpack`]) all files from a compressed tarball named `archive.tar.gz` located in the current working directory to the same location.

```rust,editable,no_run
{#include ../../../deps/examples/tar-decompress.rs}
```

## Compress a directory into tarball

[![flate2-badge]][flate2] [![tar-badge]][tar] [![cat-compression-badge]][cat-compression]

Compress `/var/log` directory into `archive.tar.gz`.

Creates a [`File`] wrapped in [`GzEncoder`] and [`tar::Builder`]. </br>Adds contents of `/var/log` directory recursively into the archive
under `backup/logs`path with [`Builder::append_dir_all`]. [`GzEncoder`] is responsible for transparently compressing the data prior to writing it into `archive.tar.gz`.

```rust,editable,no_run
{#include ../../../deps/examples/tar-compress.rs}
```

## Decompress a tarball while removing a prefix from the paths

[![flate2-badge]][flate2] [![tar-badge]][tar] [![cat-compression-badge]][cat-compression]

Iterate over the [`Archive::entries`].  Use [`Path::strip_prefix`] to remove the specified path prefix (`bundle/logs`).  Finally, extract the [`tar::Entry`] via [`Entry::unpack`].

```rust,editable,no_run
{#include ../../../deps/examples/tar-strip-prefix.rs}
```

[`Archive::unpack`]: https://docs.rs/tar/*/tar/struct.Archive.html#method.unpack
[`GzDecoder`]: https://docs.rs/flate2/*/flate2/read/struct.GzDecoder.html
[`Builder::append_dir_all`]: https://docs.rs/tar/*/tar/struct.Builder.html#method.append_dir_all
[`File`]: https://doc.rust-lang.org/std/fs/struct.File.html
[`GzEncoder`]: https://docs.rs/flate2/*/flate2/write/struct.GzEncoder.html
[`tar::Builder`]: https://docs.rs/tar/*/tar/struct.Builder.html
[`Archive::entries`]: https://docs.rs/tar/*/tar/struct.Archive.html#method.entries
[`Entry::unpack`]: https://docs.rs/tar/*/tar/struct.Entry.html#method.unpack
[`Path::strip_prefix`]: https://doc.rust-lang.org/std/path/struct.Path.html#method.strip_prefix
[`tar::Entry`]: https://docs.rs/tar/*/tar/struct.Entry.html
{{#include ../refs/link-refs.md}}
