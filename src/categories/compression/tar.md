# Working with Tarballs

{{#include tar.incl.md}}

## Decompress a tarball

[![flate2][flate2-badge]][flate2]  [![tar][tar-badge]][tar]  [![cat-compression][cat-compression-badge]][cat-compression] {{hi:Compression}}

Decompress ([`{{i:GzDecoder}}`][flate2::read::GzDecoder]⮳) and extract ([`{{i:Archive::unpack}}`][tar::Archive::unpack]⮳) all files from a compressed tarball named `archive.tar.gz` located in the current working directory to the same location.

```rust,editable,no_run
{{#include ../../../deps/tests/tar-decompress.rs}}
```

## Compress a directory into tarball

[![flate2][flate2-badge]][flate2]  [![tar][tar-badge]][tar]  [![cat-compression][cat-compression-badge]][cat-compression]

Compress `/var/log` directory into `archive.tar.gz`.

Creates a [`{{i:File}}`][std::fs::File]⮳ wrapped in [`{{i:GzEncoder}}`][flate2::write::GzEncoder]⮳ and [`{{i:tar::Builder}}`][tar::Builder]⮳

Adds contents of `/var/log` directory recursively into the archive under `backup/logs`path with [`{{i:Builder::append_dir_all}}`][tar::Builder::append_dir_all]⮳. [`{{i:GzEncoder}}`][flate2::write::GzEncoder]⮳ is responsible for transparently compressing the data prior to writing it into `archive.tar.gz`.

```rust,editable,no_run
{{#include ../../../deps/tests/tar-compress.rs}}
```

## Decompress a tarball while removing a prefix from the paths

[![flate2][flate2-badge]][flate2]  [![tar][tar-badge]][tar]  [![cat-compression][cat-compression-badge]][cat-compression]

Iterate over the [`{{i:Archive::entries}}`][tar::Archive::entries]⮳. Use [`{{i:Path::strip_prefix}}`][std::path::Path::strip_prefix]⮳ to remove the specified path prefix (`bundle/logs`). Finally, extract the [`{{i:tar::Entry}}`][tar::Entry]⮳ via [`{{i:Entry::unpack}}`][tar::Entry::unpack]⮳.

```rust,editable,no_run
{{#include ../../../deps/tests/tar-strip-prefix.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
