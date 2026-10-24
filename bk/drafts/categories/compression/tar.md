<!-- TODO Reviewed ??-??-2026 -->
# Working with Tarballs

{{#include tar.incl.md}}

The examples below read and write local filesystem paths, so their code blocks are marked `noplayground`. Prepare the source directory or archive at the paths shown before running an example.

Tar groups files and metadata but does not compress them. A `.tar.gz` file combines a tar archive with gzip compression. Other combinations include tar with [`zstd`][c~zstd~docs]↗, [`xz2`][c~xz2~docs]↗, or [`bzip2`][c~bzip2~docs]↗. ZIP is a separate archive format, provided by [`zip`][c~zip~docs]↗.

The default `flate2` backend uses Rust code. Some alternatives involve native code: `zstd` builds its C library, `xz2` binds to `liblzma`, and `bzip2` offers an optional native `libbz2` backend alongside a Rust backend. Those configurations may require a C compiler or target libraries when cross-compiling; check the selected backend and features.

## Decompress a Tarball {#decompress-a-tarball}

[![flate2][c~flate2~docs~badge]][c~flate2~docs] [![flate2~crates.io][c~flate2~crates.io~badge]][c~flate2~crates.io] [![flate2~repo][c~flate2~repo~badge]][c~flate2~repo]{{hi:flate2}} [![tar][c~tar~docs~badge]][c~tar~docs] [![tar~crates.io][c~tar~crates.io~badge]][c~tar~crates.io] [![tar~repo][c~tar~repo~badge]][c~tar~repo]{{hi:tar}} [![cat~compression][cat~compression~badge]][cat~compression]{{hi:Compression}}

Read a gzip-compressed tar archive with [`flate2::read::GzDecoder`][c~flate2::read::GzDecoder~docs]↗{{hi:flate2::read::GzDecoder}}, then extract its entries with [`tar::Archive::unpack`][c~tar::Archive::unpack~docs]↗{{hi:tar::Archive::unpack}}. The example reads the archive created by the compression example from the system temporary directory and extracts files into an `unpacked` subdirectory there. The decompression helper accepts explicit archive and destination paths. Only extract archives you trust.

```rust,editable,noplayground
{{#include ../../../crates/cats/compression/examples/tar/tar_decompress.rs:example}}
```

## Compress a Directory into a Tarball {#compress-a-directory-into-a-tarball}

[![flate2][c~flate2~docs~badge]][c~flate2~docs] [![flate2~crates.io][c~flate2~crates.io~badge]][c~flate2~crates.io] [![flate2~repo][c~flate2~repo~badge]][c~flate2~repo]{{hi:flate2}} [![tar][c~tar~docs~badge]][c~tar~docs] [![tar~crates.io][c~tar~crates.io~badge]][c~tar~crates.io] [![tar~repo][c~tar~repo~badge]][c~tar~repo]{{hi:tar}} [![cat~compression][cat~compression~badge]][cat~compression]{{hi:Compression}}

The example creates a small source directory in the system temporary directory and writes `archive.tar.gz` there. Adapt `compress_directory` to use your own source and archive paths.

Creates a [`std::fs::File`][c~std::fs::File~docs]↗{{hi:std::fs::File}} wrapped in [`flate2::write::GzEncoder`][c~flate2::write::GzEncoder~docs]↗{{hi:flate2::write::GzEncoder}} and [`tar::Builder`][c~tar::Builder~docs]↗{{hi:tar::Builder}}.

Adds the source directory recursively under the archive path `backup/logs` with [`tar::Builder::append_dir_all`][c~tar::Builder::append_dir_all~docs]↗{{hi:tar::Builder::append_dir_all}}. [`flate2::write::GzEncoder`][c~flate2::write::GzEncoder~docs]↗{{hi:flate2::write::GzEncoder}} compresses the tar stream as it is written to `archive.tar.gz`.

```rust,editable,noplayground
{{#include ../../../crates/cats/compression/examples/tar/tar_compress.rs:example}}
```

## Decompress a Tarball While Removing a Prefix from the Paths {#decompress-a-tarball-removing-prefix}

[![flate2][c~flate2~docs~badge]][c~flate2~docs] [![flate2~crates.io][c~flate2~crates.io~badge]][c~flate2~crates.io] [![flate2~repo][c~flate2~repo~badge]][c~flate2~repo]{{hi:flate2}} [![tar][c~tar~docs~badge]][c~tar~docs] [![tar~crates.io][c~tar~crates.io~badge]][c~tar~crates.io] [![tar~repo][c~tar~repo~badge]][c~tar~repo]{{hi:tar}} [![cat~compression][cat~compression~badge]][cat~compression]{{hi:Compression}}

Iterate over [`tar::Archive::entries`][c~tar::Archive::entries~docs]↗{{hi:tar::Archive::entries}} and use [`std::path::Path::strip_prefix`][c~std::path::Path::strip_prefix~docs]↗{{hi:std::path::Path::strip_prefix}} to remove `bundle/logs` from each entry path. Then extract each [`tar::Entry`][c~tar::Entry~docs]↗{{hi:tar::Entry}} with [`tar::Entry::unpack`][c~tar::Entry::unpack~docs]↗{{hi:tar::Entry::unpack}}.

```rust,editable,noplayground
{{#include ../../../crates/cats/compression/examples/tar/tar_strip_prefix.rs:example}}
```

## Related Topics {#related-topics .skip}

- [Compression and decompression](compression.md).
- [Compression formats overview](index.md).
- [[filesystem | Filesystem]].
- [Temporary files and directories](../../../src/categories/filesystem/temporary_files_and_directories.md).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[tar: review](https://github.com/john-cd/rust_howto/issues/253)
</div>
