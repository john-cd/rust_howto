# Working with Tarballs

{{#include tar.incl.md}}

## Decompress a tarball

[![flate2][c-flate2-badge]][c-flate2]  [![tar][c-tar-badge]][c-tar]  [![cat-compression][cat-compression-badge]][cat-compression] {{hi:Compression}}

Decompress ({{hi:GzDecoder}}[`GzDecoder`][c-flate2::read::GzDecoder]⮳) and extract ({{hi:Archive::unpack}}[`Archive::unpack`][c-tar::Archive::unpack]⮳) all files from a compressed tarball named `archive.tar.gz` located in the current working directory to the same location.

```rust,editable,no_run
{{#include ../../../deps/tests/tar-decompress.rs}}
```

## Compress a directory into tarball

[![flate2][c-flate2-badge]][c-flate2]  [![tar][c-tar-badge]][c-tar]  [![cat-compression][cat-compression-badge]][cat-compression]

Compress `/var/log` directory into `archive.tar.gz`.

Creates a {{hi:File}}[`File`][c-std::fs::File]⮳ wrapped in {{hi:GzEncoder}}[`GzEncoder`][c-flate2::write::GzEncoder]⮳ and {{hi:tar::Builder}}[`tar::Builder`][c-tar::Builder]⮳

Adds contents of `/var/log` directory recursively into the archive under `backup/logs`path with {{hi:Builder::append_dir_all}}[`Builder::append_dir_all`][c-tar::Builder::append_dir_all]⮳. {{hi:GzEncoder}}[`GzEncoder`][c-flate2::write::GzEncoder]⮳ is responsible for transparently compressing the data prior to writing it into `archive.tar.gz`.

```rust,editable,no_run
{{#include ../../../deps/tests/tar-compress.rs}}
```

## Decompress a tarball while removing a prefix from the paths

[![flate2][c-flate2-badge]][c-flate2]  [![tar][c-tar-badge]][c-tar]  [![cat-compression][cat-compression-badge]][cat-compression]

Iterate over the {{hi:Archive::entries}}[`Archive::entries`][c-tar::Archive::entries]⮳. Use {{hi:Path::strip_prefix}}[`Path::strip_prefix`][c-std::path::Path::strip_prefix]⮳ to remove the specified path prefix (`bundle/logs`). Finally, extract the {{hi:tar::Entry}}[`tar::Entry`][c-tar::Entry]⮳ via {{hi:Entry::unpack}}[`Entry::unpack`][c-tar::Entry::unpack]⮳.

```rust,editable,no_run
{{#include ../../../deps/tests/tar-strip-prefix.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
