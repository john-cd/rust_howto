# Reading & Writing from Files

{{#include read_write.incl.md}}

ile reading and writing primarily through the std::fs module and the traits found in std::io, like Read and Write.

## Read Lines of Strings from a File {#read-lines-of-strings-from-a-file}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}} [![cat~filesystem][cat~filesystem~badge]][cat~filesystem]{{hi:Filesystem}}

The following writes a three-line message to a file, then reads it back a line at a time with the [`std::io::Lines`][c~std::io::Lines~docs]↗{{hi:std::io::Lines}} iterator created by [`std::io::BufRead::lines`][c~std::io::BufRead::lines~docs]↗.{{hi:std::io::BufRead::lines}}

[`std::fs::File`][c~std::fs::File~docs]↗{{hi:std::fs::File}} implements [`std::io::Read`][c~std::io::Read~docs]↗{{hi:std::io::Read}}, which provides the [`std::io::BufReader`][c~std::io::BufReader~docs]↗{{hi:std::io::BufReader}} trait. [`std::fs::File::create`][c~std::fs::File::create~docs]↗{{hi:std::fs::File::create}} opens a [`std::fs::File`][c~std::fs::File~docs]↗{{hi:std::fs::File}} for writing, [`std::fs::File::open`][c~std::fs::File::open~docs]↗{{hi:std::fs::File::open}} for reading.

```rust,editable
{{#include ../../../crates/cats/filesystem/examples/read_write/read_file.rs:example}}
```

## Avoid Writing and Reading from the Same File {#avoid-writing-and-reading-from-the-same-file}

[![same-file][c~same-file~docs~badge]][c~same-file~docs]{{hi:same-file}} [![cat~filesystem][cat~filesystem~badge]][cat~filesystem]{{hi:Filesystem}}

This example demonstrates how to check whether two files refer to the same file or directory, using [`same-file::Handle`][c~same-file::Handle~docs]↗{{hi:same-file::Handle}}. Use this technique to check whether the standard input and standard output (or error) of a process are the same (due to piping / redirection) and avoid reading and writing from the same file. You may also use `same-file` check whether two file paths correspond to the same file:

```rust,editable
{{#include ../../../crates/cats/filesystem/examples/read_write/same_file.rs:example}}
```

## Access a File Randomly Using a Memory Map {#memory-map}

[![memmap2][c~memmap2~docs~badge]][c~memmap2~docs]{{hi:memmap2}} [![cat~filesystem][cat~filesystem~badge]][cat~filesystem]{{hi:Filesystem}}

In the following example, we create a memory map{{hi:Memory map}} of a file using [`memmap2`][c~memmap2~docs]↗{{hi:memmap2}} and simulate some non-sequential reads{{hi:Non-sequential reads}} from the file. Using a memory map simplifies navigation of a [`std::fs::File`][c~std::fs::File~docs]↗{{hi:std::fs::File}} - by indexing into a slice rather than dealing with [`std::fs::File::seek`][c~std::fs::File::seek~docs]↗.{{hi:std::fs::File::seek}}Memory mapping may also enhance performance in applications that require large data processing or frequent file access.

Note that the [`memmap2::Mmap::map`][c~memmap2::Mmap::map~docs]↗{{hi:memmap2::Mmap::map}} function assumes the file behind the memory map is not being modified at the same time by another process or else [a race condition][wikipedia~race-condition]↗ may occur.

It is also possible to write to a `File` as if it were a `&mut [u8]` by using mutable memory maps.

```rust,editable
{{#include ../../../crates/cats/filesystem/examples/read_write/memmap2.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
</div>
