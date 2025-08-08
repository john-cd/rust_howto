# Read & Write from Files

{{#include read-write.incl.md}}

## Read Lines of Strings from a File {#read-lines-of-strings-from-a-file}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}} [![cat~filesystem][cat~filesystem~badge]][cat~filesystem]{{hi:Filesystem}}

Writes a three-line message to a file, then reads it back a line at a time with the [`std::io::Lines`][c~std::io::Lines~docs]{{hi:std::io::Lines}}↗ iterator created by
[`std::io::BufRead::lines`][c~std::io::BufRead::lines~docs]{{hi:std::io::BufRead::lines}}↗ [`std::fs::File`][c~std::fs::File~docs]{{hi:std::fs::File}}↗ implements [`std::io::Read`][c~std::io::Read~docs]{{hi:std::io::Read}}↗ which provides [`std::io::BufReader`][c~std::io::BufReader~docs]{{hi:std::io::BufReader}}↗ trait. [`std::fs::File::create`][c~std::fs::File::create~docs]{{hi:std::fs::File::create}}↗ opens a [`std::fs::File`][c~std::fs::File~docs]{{hi:std::fs::File}}↗ for writing, [`std::fs::File::open`][c~std::fs::File::open~docs]{{hi:std::fs::File::open}}↗ for reading.

```rust,editable
{{#include ../../../crates/cats/filesystem/examples/read_write/read_file.rs:example}}
```

## Avoid Writing and Reading from the Same File {#avoid-writing-and-reading-from-the-same-file}

[![same-file][c~same-file~docs~badge]][c~same-file~docs]{{hi:same-file}} [![cat~filesystem][cat~filesystem~badge]][cat~filesystem]{{hi:Filesystem}}

Use [`same-file::Handle`][c~same-file::Handle~docs]{{hi:same-file::Handle}}↗ to a file that can be tested for equality with other handles. In this example, the handles of file to be read from and to be written to are tested for equality.

```rust,editable
{{#include ../../../crates/cats/filesystem/examples/read_write/same_file.rs:example}}
```

```bash
cargo run
```

displays the contents of the file `new.txt`.

```bash
cargo run >> new.txt
```

errors because the two files are same.

## Access a File Randomly Using a Memory Map {#memory-map}

[![memmap2][c~memmap2~docs~badge]][c~memmap2~docs]{{hi:memmap2}} [![cat~filesystem][cat~filesystem~badge]][cat~filesystem]{{hi:Filesystem}}

Creates a memory map{{hi:Memory map}} of a file using [`memmap2`][c~memmap2~docs]{{hi:memmap2}}↗ and simulates some non-sequential reads{{hi:Non-sequential reads}} from the file. Using a memory map means you just index into a slice rather than dealing with [`std::fs::File::seek`][c~std::fs::File::seek~docs]{{hi:std::fs::File::seek}}↗ to navigate a [`std::fs::File`][c~std::fs::File~docs]{{hi:std::fs::File}}↗.

The [`memmap2::Mmap::map`][c~memmap2::Mmap::map~docs]{{hi:memmap2::Mmap::map}}↗ function assumes the file behind the memory map is not being modified at the same time by another process or else [a race condition][wikipedia~race-condition]↗ occurs.

```rust,editable
{{#include ../../../crates/cats/filesystem/examples/read_write/memmap2.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[final review](https://github.com/john-cd/rust_howto/issues/932)
</div>
