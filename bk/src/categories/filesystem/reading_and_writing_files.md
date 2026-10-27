<!-- TODO Reviewed ??-??-2026 -->

# Reading and Writing Files

{{#include reading_and_writing_files.incl.md}}

File reading and writing happens primarily through the `std::fs` module of the standard library, and the traits found in `std::io`, like `Read` and `Write`.

## Read from and Write to a File {#read-and-write}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}} [![cat~filesystem][cat~filesystem~badge]][cat~filesystem]{{hi:Filesystem}}

The following example opens a file, writes to it, then reads its contents back in several different ways.

- [`std::fs::File::create`][c~std::fs::File::create~docs]↗{{hi:std::fs::File::create}} opens a [`std::fs::File`][c~std::fs::File~docs]↗{{hi:std::fs::File}} for writing, [`std::fs::File::open`][c~std::fs::File::open~docs]↗{{hi:std::fs::File::open}} for reading. `std::fs::OpenOptions` may be used to open a file in another mode.
- `File` implements the [`std::io::Read`][c~std::io::Read~docs]↗{{hi:std::io::Read}} trait for reading bytes from a source, the [`std::io::Write`][c~std::io::Write~docs]↗{{hi:std::io::Write}} trait for writing bytes to a destination, and `std::io::Seek` for moving within a stream of bytes.
- You may use the `std::write!` and `writeln!` macros to write formatted data.
- `File` does not buffer reads and writes. Wrap it in a [`std::io::BufReader`][c~std::io::BufReader~docs]↗{{hi:std::io::BufReader}} or [`std::io::BufWriter`][c~std::io::BufWriter~docs]↗{{hi:std::io::BufWriter}} when performing many small read or write calls. `BufReader` implements `BufRead`; `BufWriter` implements `Write`.
- `std::io::BufRead::lines` returns a [`std::io::Lines`][c~std::io::Lines~docs]↗{{hi:std::io::Lines}} iterator over individual lines of the file. LF or CRLF line endings are removed.
- `std::fs::write` writes a slice as the entire contents of a file. This is a convenience function for using `File::create` and `std::io::Write::write_all` with fewer imports.
- `std::fs::read` and `std::fs::read_to_string` read the entire contents of a file into a bytes vector or a string, respectively. They are convenience functions for using `File::open` and `std::io::Read::read_to_end` or `read_to_string` without an intermediate variable.

```rust,editable
{{#include ../../../crates/cats/filesystem/examples/read_write/read_file.rs:example}}
```

## Avoid Writing and Reading from the Same File {#avoid-writing-and-reading-from-the-same-file}

[![same-file][c~same-file~docs~badge]][c~same-file~docs]{{hi:same-file}} [![cat~filesystem][cat~filesystem~badge]][cat~filesystem]{{hi:Filesystem}}

This example demonstrates how to check whether two files refer to the same file or directory, using [`same-file::Handle`][c~same-file::Handle~docs]↗{{hi:same-file::Handle}}. Use this technique to check whether the standard input and standard output (or error) of a process are the same (due to piping / redirection) and avoid reading and writing from the same file. You can also use `same-file` to check whether two file paths correspond to the same file:

```rust,editable
{{#include ../../../crates/cats/filesystem/examples/read_write/same_file.rs:example}}
```

## Access a File Randomly Using a Memory Map {#memory-map}

[![memmap2][c~memmap2~docs~badge]][c~memmap2~docs]{{hi:memmap2}} [![cat~filesystem][cat~filesystem~badge]][cat~filesystem]{{hi:Filesystem}}

In the following example, we create a memory map{{hi:Memory map}} of a file using [`memmap2`][c~memmap2~docs]↗{{hi:memmap2}} and simulate some non-sequential reads{{hi:Non-sequential reads}} from the file. Using a memory map simplifies navigation of a [`std::fs::File`][c~std::fs::File~docs]↗{{hi:std::fs::File}} - by indexing into a slice rather than dealing with [`std::fs::File::seek`][c~std::fs::File::seek~docs]↗.{{hi:std::fs::File::seek}}Memory mapping may also enhance performance in applications that require large data processing or frequent file access.

Note that the [`memmap2::Mmap::map`][c~memmap2::Mmap::map~docs]↗{{hi:memmap2::Mmap::map}} function assumes the file behind the memory map is not being modified at the same time by another process or else [a race condition][wikipedia~race-condition]↗ may occur.

It is also possible to write to a `File` as if it were a `&mut [u8]` by using mutable memory maps.

```rust,editable,noplayground
{{#include ../../../crates/cats/filesystem/examples/read_write/memmap2.rs:example}}
```

## Replace a File Atomically {#replace-a-file-atomically}

[![tempfile][c~tempfile~docs~badge]][c~tempfile~docs]{{hi:tempfile}} [![atomicwrites][c~atomicwrites~docs~badge]][c~atomicwrites~docs]{{hi:atomicwrites}} [![cat~filesystem][cat~filesystem~badge]][cat~filesystem]{{hi:Filesystem}}

Overwriting a file in place is not crash-safe: if the process or the machine dies halfway, readers find a truncated or half-written file. The robust pattern writes the new contents to a temporary file, flushes it to disk, then renames it over the target. The rename is atomic, so readers see either the old file or the new one.

The example shows two variants: [`tempfile::NamedTempFile`][c~tempfile~docs]↗{{hi:tempfile::NamedTempFile}} with `persist`, and [`atomicwrites::AtomicFile`][c~atomicwrites~docs]↗{{hi:atomicwrites::AtomicFile}}, which wraps the same steps.

- Create the temporary file in the **same directory** as the target. Atomic rename requires the same filesystem; `persist` fails across filesystems.
- Call `sync_all` **before** the rename. Without it, a crash may leave the new name pointing at empty or partial data.
- Synchronize the parent directory **after** the rename on Unix to persist the directory entry. [`persist` does not synchronize file contents or the parent directory itself][tempfile~persist-durability]. The example reports a directory-sync failure even though replacement has already happened.
- On Windows, `persist` replaces an existing target, but the replace fails if another process holds the target open without sharing; retry on transient errors if other programs may read the file.
- If the target is a symbolic link, resolve it first (see [Canonicalize a Path][ex~filesystem~canonicalize]), or the rename replaces the link rather than the file it points to.

Atomic visibility and durability after power loss are different guarantees. The standard library does not provide a portable directory-sync equivalent on Windows, so the tempfile variant does not promise complete power-loss durability there. Its ordinary replacement tests check final contents and cleanup, not recovery after a system crash.

```rust,editable,noplayground
{{#include ../../../crates/cats/filesystem/examples/atomic_write/atomic_write.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[file_watching | File Watching]].
- [[paths | Paths]].
- [[stdin_stdout | Standard Input and Output]].
- [[tempfile | Temporary Files]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  accuracy review: file system
</div>
