# Working with Tarballs

{{#include tar.incl.md}}

## Decompress a Tarball {#decompress-a-tarball}

[![flate2][c~flate2~docs~badge]][c~flate2~docs]{{hi:flate2}} [![tar][c~tar~docs~badge]][c~tar~docs]{{hi:tar}} [![cat~compression][cat~compression~badge]][cat~compression]{{hi:Compression}}

Decompress ([`flate2::read::GzDecoder`][c~flate2::read::GzDecoder~docs]{{hi:flate2::read::GzDecoder}}⮳) and extract ([`tar::Archive::unpack`][c~tar::Archive::unpack~docs]{{hi:tar::Archive::unpack}}⮳) all files from a compressed tarball named `archive.tar.gz` located in the current working directory to the same location.

```rust,editable,noplayground
{{#include ../../../crates/cats/compression/examples/tar/tar_decompress.rs:example}}
```

## Compress a Directory into a Tarball {#compress-a-directory-into-a-tarball}

[![flate2][c~flate2~docs~badge]][c~flate2~docs]{{hi:flate2}} [![tar][c~tar~docs~badge]][c~tar~docs]{{hi:tar}} [![cat~compression][cat~compression~badge]][cat~compression]{{hi:Compression}}

Compress `/var/log` directory into `archive.tar.gz`.

Creates a [`std::fs::File`][c~std::fs::File~docs]{{hi:std::fs::File}}⮳ wrapped in [`flate2::write::GzEncoder`][c~flate2::write::GzEncoder~docs]{{hi:flate2::write::GzEncoder}}⮳ and [`tar::Builder`][c~tar::Builder~docs]{{hi:tar::Builder}}⮳.

Adds contents of `/var/log` directory recursively into the archive under `backup/logs`path with [`tar::Builder::append_dir_all`][c~tar::Builder::append_dir_all~docs]{{hi:tar::Builder::append_dir_all}}⮳. [`flate2::write::GzEncoder`][c~flate2::write::GzEncoder~docs]{{hi:flate2::write::GzEncoder}}⮳ is responsible for transparently compressing the data prior to writing it into `archive.tar.gz`.

```rust,editable,noplayground
{{#include ../../../crates/cats/compression/examples/tar/tar_compress.rs:example}}
```

## Decompress a Tarball While Removing a Prefix from the Paths {#decompress-a-tarball-removing-prefix}

[![flate2][c~flate2~docs~badge]][c~flate2~docs]{{hi:flate2}} [![tar][c~tar~docs~badge]][c~tar~docs]{{hi:tar}} [![cat~compression][cat~compression~badge]][cat~compression]{{hi:Compression}}

Iterate over the [`tar::Archive::entries`][c~tar::Archive::entries~docs]{{hi:tar::Archive::entries}}⮳. Use [`std::path::Path::strip_prefix`][c~std::path::Path::strip_prefix~docs]{{hi:std::path::Path::strip_prefix}}⮳ to remove the specified path prefix (`bundle/logs`). Finally, extract the [`tar::Entry`][c~tar::Entry~docs]{{hi:tar::Entry}}⮳ via [`tar::Entry::unpack`][c~tar::Entry::unpack~docs]{{hi:tar::Entry::unpack}}⮳.

```rust,editable,noplayground
{{#include ../../../crates/cats/compression/examples/tar/tar_strip_prefix.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[tar: review](https://github.com/john-cd/rust_howto/issues/253) tar_decompress.rs is noplayground - fix? tar_compress.rs is noplayground - fix? tar_strip_prefix.rs is noplayground - fix?
</div>
