# Read & Write

{{#include read-write.incl.md}}

## Read lines of strings from a file

[![std][std-badge]][std]  [![cat-filesystem][cat-filesystem-badge]][cat-filesystem]

Writes a three-line message to a file, then reads it back a line at a time with the [`Lines`][std::io::Lines]⮳ iterator created by
[`BufRead::lines`][std::io::BufRead::lines]⮳  [`File`][std::fs::File]⮳ implements [`Read`][std::io::Read]⮳ which provides [`BufReader`][std::io::BufReader]⮳ trait. [`File::create`][std::fs::File::create]⮳ opens a [`File`][std::fs::File]⮳ for writing, [`File::open`][std::fs::File::open]⮳ for reading.

```rust,editable
{{#include ../../../deps/tests/read-file.rs}}
```

## Avoid writing and reading from a same file

[![same-file][same-file-badge]][same-file]  [![cat-filesystem][cat-filesystem-badge]][cat-filesystem]

Use [`same-file::Handle`][same-file::Handle]⮳ to a file that can be tested for equality with other handles. In this example, the handles of file to be read from and to be written to are tested for equality.

```rust,editable,no_run
{{#include ../../../deps/tests/same-file.rs}}
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

[![memmap][memmap-badge]][memmap]  [![cat-filesystem][cat-filesystem-badge]][cat-filesystem]

Creates a {{i:memory map}} of a file using [`memmap`][memmap]⮳ and simulates some {{i:non-sequential reads}} from the file. Using a memory map means you just index into a slice rather than dealing with [`seek`][std::fs::File::seek]⮳ to navigate a [`File`][std::fs::File]⮳.

The [`Mmap::map`][memmap::Mmap::map]⮳ function assumes the file behind the memory map is not being modified at the same time by another process or else [a race condition][wikipedia-race-condition]⮳ occurs.

```rust,editable
{{#include ../../../deps/tests/memmap.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
