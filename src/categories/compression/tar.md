# Working with Tarballs

{{#include tar.incl.md}}

## Decompress a tarball

[![flate2][flate2-badge]][flate2]  [![tar][tar-badge]][tar]  [![cat-compression][cat-compression-badge]][cat-compression]

Decompress ([`GzDecoder`][flate2::read::GzDecoder]⮳) and extract ([`Archive::unpack`][tar::Archive::unpack]⮳) all files from a compressed tarball named `archive.tar.gz` located in the current working directory to the same location.

```rust,editable,no_run
{{#include ../../../deps/tests/tar-decompress.rs}}
```

## Compress a directory into tarball

[![flate2][flate2-badge]][flate2]  [![tar][tar-badge]][tar]  [![cat-compression][cat-compression-badge]][cat-compression]

Compress `/var/log` directory into `archive.tar.gz`.

Creates a [`File`][std::fs::File]⮳ wrapped in [`GzEncoder`][flate2::write::GzEncoder]⮳ and [`tar::Builder`][tar::Builder]⮳

Adds contents of `/var/log` directory recursively into the archive under `backup/logs`path with [`Builder::append_dir_all`][tar::Builder::append_dir_all]⮳. [`GzEncoder`][flate2::write::GzEncoder]⮳ is responsible for transparently compressing the data prior to writing it into `archive.tar.gz`.

```rust,editable,no_run
{{#include ../../../deps/tests/tar-compress.rs}}
```

## Decompress a tarball while removing a prefix from the paths

[![flate2][flate2-badge]][flate2]  [![tar][tar-badge]][tar]  [![cat-compression][cat-compression-badge]][cat-compression]

Iterate over the [`Archive::entries`][tar::Archive::entries]⮳. Use [`Path::strip_prefix`][std::path::Path::strip_prefix]⮳ to remove the specified path prefix (`bundle/logs`). Finally, extract the [`tar::Entry`][tar::Entry]⮳ via [`Entry::unpack`][tar::Entry::unpack]⮳.

```rust,editable,no_run
{{#include ../../../deps/tests/tar-strip-prefix.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
