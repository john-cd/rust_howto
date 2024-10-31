# Read & Write

{{#include read-write.incl.md}}

## Read lines of strings from a file

[![std][c-std-badge]][c-std]{{hi:std}}  [![cat-filesystem][cat-filesystem-badge]][cat-filesystem]{{hi:Filesystem}}

Writes a three-line message to a file, then reads it back a line at a time with the [`std::io::Lines`][c-std::io::Lines]{{hi:std::io::Lines}}⮳ iterator created by
[`std::io::BufRead::lines`][c-std::io::BufRead::lines]{{hi:std::io::BufRead::lines}}⮳  [`std::fs::File`][c-std::fs::File]{{hi:std::fs::File}}⮳ implements [`std::io::Read`][c-std::io::Read]{{hi:std::io::Read}}⮳ which provides [`std::io::BufReader`][c-std::io::BufReader]{{hi:std::io::BufReader}}⮳ trait. [`std::fs::File::create`][c-std::fs::File::create]{{hi:std::fs::File::create}}⮳ opens a [`std::fs::File`][c-std::fs::File]{{hi:std::fs::File}}⮳ for writing, [`std::fs::File::open`][c-std::fs::File::open]{{hi:std::fs::File::open}}⮳ for reading.

```rust
{{#include ../../../deps/tests/read_file.rs}}
```

## Avoid writing and reading from a same file

[![same-file][c-same_file-badge]][c-same_file]{{hi:same-file}}  [![cat-filesystem][cat-filesystem-badge]][cat-filesystem]{{hi:Filesystem}}

Use [`same_file::Handle`][c-same_file::Handle]{{hi:same_file::Handle}}⮳ to a file that can be tested for equality with other handles. In this example, the handles of file to be read from and to be written to are tested for equality.

```rust
{{#include ../../../deps/tests/same_file.rs}}
```

```bash
cargo run
```

displays the contents of the file `new.txt`.

```bash
cargo run >> new.txt
```

errors because the two files are same.

## Access a file randomly using a memory map

[![memmap][c-memmap-badge]][c-memmap]{{hi:memmap}}  [![cat-filesystem][cat-filesystem-badge]][cat-filesystem]{{hi:Filesystem}}

Creates a memory map{{hi:Memory map}} of a file using [`memmap`][c-memmap]{{hi:memmap}}⮳ and simulates some non-sequential reads{{hi:Non-sequential reads}} from the file. Using a memory map means you just index into a slice rather than dealing with [`std::fs::File::seek`][c-std::fs::File::seek]{{hi:std::fs::File::seek}}⮳ to navigate a [`std::fs::File`][c-std::fs::File]{{hi:std::fs::File}}⮳.

The [`memmap::Mmap::map`][c-memmap::Mmap::map]{{hi:memmap::Mmap::map}}⮳ function assumes the file behind the memory map is not being modified at the same time by another process or else [a race condition][wikipedia-race-condition]⮳ occurs.

```rust
{{#include ../../../deps/tests/memmap.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO change to memmap2
The older memmap crate is unmaintained.

[![memmap2][c-memmap2-badge]][c-memmap2]{{hi:memmap2}}
[![memmap2-crates.io][c-memmap2-crates.io-badge]][c-memmap2-crates.io]
[![memmap2-github][c-memmap2-github-badge]][c-memmap2-github]
[![memmap2-lib.rs][c-memmap2-lib.rs-badge]][c-memmap2-lib.rs]
</div>
