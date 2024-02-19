# Working with Tarballs

## Decompress a tarball

[![flate2-badge]][flate2] [![tar-badge]][tar] [![cat-compression-badge]][cat-compression]

Decompress ([`GzDecoder`][GzDecoder]) and extract ([`Archive::unpack`][Archive::unpack]) all files from a compressed tarball named `archive.tar.gz` located in the current working directory to the same location.

```rust,editable,no_run
{{#include ../../deps/examples/tar-decompress.rs}}
```

## Compress a directory into tarball

[![flate2-badge]][flate2] [![tar-badge]][tar] [![cat-compression-badge]][cat-compression]

Compress `/var/log` directory into `archive.tar.gz`.

Creates a [`File`][File] wrapped in [`GzEncoder`][GzEncoder] and [`tar::Builder`][tar::Builder] </br>Adds contents of `/var/log` directory recursively into the archive under `backup/logs`path with [`Builder::append_dir_all`][Builder::append_dir_all]. [`GzEncoder`][GzEncoder] is responsible for transparently compressing the data prior to writing it into `archive.tar.gz`.

```rust,editable,no_run
{{#include ../../deps/examples/tar-compress.rs}}
```

## Decompress a tarball while removing a prefix from the paths

[![flate2-badge]][flate2] [![tar-badge]][tar] [![cat-compression-badge]][cat-compression]

Iterate over the [`Archive::entries`][Archive::entries]. Use [`Path::strip_prefix`][Path::strip_prefix] to remove the specified path prefix (`bundle/logs`). Finally, extract the [`tar::Entry`][tar::Entry] via [`Entry::unpack`][Entry::unpack].

```rust,editable,no_run
{{#include ../../deps/examples/tar-strip-prefix.rs}}
```

{{#include ../refs/link-refs.md}}
