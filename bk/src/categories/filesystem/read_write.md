# Reading & Writing from Files

{{#include read_write.incl.md}}

File reading and writing happens primarily through the `std::fs` module of the standard library, and the traits found in `std::io`, like `Read` and `Write`.

## Read from and Write to a File {#read-and-write}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}} [![cat~filesystem][cat~filesystem~badge]][cat~filesystem]{{hi:Filesystem}}

The following example opens a file, writes to it, then reads its contents back in several different ways.

- [`std::fs::File::create`][c~std::fs::File::create~docs]↗{{hi:std::fs::File::create}} opens a [`std::fs::File`][c~std::fs::File~docs]↗{{hi:std::fs::File}} for writing, [`std::fs::File::open`][c~std::fs::File::open~docs]↗{{hi:std::fs::File::open}} for reading. `std::fs::OpenOptions` may be used to open a file in another mode.
- `File` implements the [`std::io::Read`][c~std::io::Read~docs]↗{{hi:std::io::Read}} trait for reading bytes from a source, the [`std::io::Write`][c~std::io::Write~docs]↗{{hi:std::io::Write}} trait for writing bytes to a destination, and `std::io::Seek` for moving within a stream of bytes.
- You may use the `std::write!` and `writeln!` macros to write formatted data.
- `File` does not buffer reads and writes. Wrap the `File` in a [`std::io::BufReader`][c~std::io::BufReader~docs]↗{{hi:std::io::BufReader}} or `BufWriter` (which implement `BufRead` and `BufWrite`), when performing many small read or write calls.
- `std::io::BufRead::lines` returns a [`std::io::Lines`][c~std::io::Lines~docs]↗{{hi:std::io::Lines}} iterator over individual lines of the file. LF or CRLF line endings are removed.
- `std::fs::write` writes a slice as the entire contents of a file. This is a convenience function for using `File::create` and `std::io::Write::write_all` with fewer imports.
- `std::fs::read` and `std::fs::read_to_string` read the entire contents of a file into a bytes vector or a string, respectively. They are convenience functions for using `File::open` and `std::io::Read::read_to_end` or `read_to_string` without an intermediate variable.

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

## Related Topics {#related-topics .skip}

- [[file_watching | File Watching]].
- [[paths | Paths]].
- [[sdtin_stdout | Standard Input and Output]].
- [[tempfile | Temporary Files]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
</div>
