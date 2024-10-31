# Working with Tarballs

{{#include tar.incl.md}}

## Decompress a tarball

[![flate2][c-flate2-badge]][c-flate2]{{hi:flate2}}  [![tar][c-tar-badge]][c-tar]{{hi:tar}}  [![cat-compression][cat-compression-badge]][cat-compression]{{hi:Compression}}

Decompress ([`flate2::read::GzDecoder`][c-flate2::read::GzDecoder]{{hi:flate2::read::GzDecoder}}⮳) and extract ([`tar::Archive::unpack`][c-tar::Archive::unpack]{{hi:tar::Archive::unpack}}⮳) all files from a compressed tarball named `archive.tar.gz` located in the current working directory to the same location.

```rust
{{#include ../../../deps/tests/cats/compression/tar_decompress.rs}}
```

## Compress a directory into tarball

[![flate2][c-flate2-badge]][c-flate2]{{hi:flate2}}  [![tar][c-tar-badge]][c-tar]{{hi:tar}}  [![cat-compression][cat-compression-badge]][cat-compression]{{hi:Compression}}

Compress `/var/log` directory into `archive.tar.gz`.

Creates a [`std::fs::File`][c-std::fs::File]{{hi:std::fs::File}}⮳ wrapped in [`flate2::write::GzEncoder`][c-flate2::write::GzEncoder]{{hi:flate2::write::GzEncoder}}⮳ and [`tar::Builder`][c-tar::Builder]{{hi:tar::Builder}}⮳

Adds contents of `/var/log` directory recursively into the archive under `backup/logs`path with [`tar::Builder::append_dir_all`][c-tar::Builder::append_dir_all]{{hi:tar::Builder::append_dir_all}}⮳. [`flate2::write::GzEncoder`][c-flate2::write::GzEncoder]{{hi:flate2::write::GzEncoder}}⮳ is responsible for transparently compressing the data prior to writing it into `archive.tar.gz`.

```rust
{{#include ../../../deps/tests/cats/compression/tar_compress.rs}}
```

## Decompress a tarball while removing a prefix from the paths

[![flate2][c-flate2-badge]][c-flate2]{{hi:flate2}}  [![tar][c-tar-badge]][c-tar]{{hi:tar}}  [![cat-compression][cat-compression-badge]][cat-compression]{{hi:Compression}}

Iterate over the [`tar::Archive::entries`][c-tar::Archive::entries]{{hi:tar::Archive::entries}}⮳. Use [`std::path::Path::strip_prefix`][c-std::path::Path::strip_prefix]{{hi:std::path::Path::strip_prefix}}⮳ to remove the specified path prefix (`bundle/logs`). Finally, extract the [`tar::Entry`][c-tar::Entry]{{hi:tar::Entry}}⮳ via [`tar::Entry::unpack`][c-tar::Entry::unpack]{{hi:tar::Entry::unpack}}⮳.

```rust
{{#include ../../../deps/tests/cats/compression/tar_strip_prefix.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO: review

`flate2` uses a pure-Rust implementation by default. Use feature flags to opt in to system zlib.
</div>
