# Read & Write

{{#include read-write.incl.md}}

## Read lines of strings from a file

[![std][c-std-badge]][c-std]  [![cat-filesystem][cat-filesystem-badge]][cat-filesystem]

Writes a three-line message to a file, then reads it back a line at a time with the {{hi:Lines}}[`Lines`][c-std::io::Lines]⮳ iterator created by
{{hi:BufRead::lines}}[`BufRead::lines`][c-std::io::BufRead::lines]⮳  {{hi:File}}[`File`][c-std::fs::File]⮳ implements {{hi:Read}}[`Read`][c-std::io::Read]⮳ which provides {{hi:BufReader}}[`BufReader`][c-std::io::BufReader]⮳ trait. {{hi:File::create}}[`File::create`][c-std::fs::File::create]⮳ opens a {{hi:File}}[`File`][c-std::fs::File]⮳ for writing, {{hi:File::open}}[`File::open`][c-std::fs::File::open]⮳ for reading.

```rust,editable
{{#include ../../../deps/tests/read-file.rs}}
```

## Avoid writing and reading from a same file

[![same-file][c-same-file-badge]][c-same-file]  [![cat-filesystem][cat-filesystem-badge]][cat-filesystem]

Use {{hi:same-file::Handle}}[`same-file::Handle`][c-same-file::Handle]⮳ to a file that can be tested for equality with other handles. In this example, the handles of file to be read from and to be written to are tested for equality.

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

[![memmap][c-memmap-badge]][c-memmap]  [![cat-filesystem][cat-filesystem-badge]][cat-filesystem]

Creates a {{i:memory map}} of a file using {{hi:memmap}}[`memmap`][c-memmap]⮳ and simulates some {{i:non-sequential reads}} from the file. Using a memory map means you just index into a slice rather than dealing with {{hi:seek}}[`seek`][c-std::fs::File::seek]⮳ to navigate a {{hi:File}}[`File`][c-std::fs::File]⮳.

The {{hi:Mmap::map}}[`Mmap::map`][c-memmap::Mmap::map]⮳ function assumes the file behind the memory map is not being modified at the same time by another process or else [a race condition][wikipedia-race-condition]⮳ occurs.

```rust,editable
{{#include ../../../deps/tests/memmap.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
