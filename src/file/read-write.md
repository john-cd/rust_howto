# Read & Write

## Read lines of strings from a file

[![std-badge]][std] [![cat-filesystem-badge]][cat-filesystem]

Writes a three-line message to a file, then reads it back a line at a
time with the [`Lines`][Lines] iterator created by
`[BufRead::lines]` [`File`][File] implements [`Read`][Read] which provides `[BufReader]`
trait.  [`File::create`][File::create] opens a [`File`][File] for writing, [`File::open`][File::open] for
reading.

```rust,editable
{{#include ../../deps/examples/read-file.rs}}
```

## Avoid writing and reading from a same file

[![same-file-badge]][same-file] [![cat-filesystem-badge]][cat-filesystem]

Use [`same-file::Handle`][same-file::Handle] to a file that can be tested for equality with
other handles. In this example, the handles of file to be read from and
to be written to are tested for equality.

```rust,editable,no_run
{{#include ../../deps/examples/same-file.rs}}
```

```bash
cargo run
```

displays the contents of the file new.txt.

```bash
cargo run >> new.txt
```

errors because the two files are same.

## Access a file randomly using a memory map

[![memmap-badge]][memmap] [![cat-filesystem-badge]][cat-filesystem]

Creates a memory map of a file using [memmap] and simulates some non-sequential
reads from the file. Using a memory map means you just index into a slice rather
than dealing with [`seek`][seek] to navigate a File.

The [`Mmap::map`][Mmap::map] function assumes the file
behind the memory map is not being modified at the same time by another process
or else a [race condition] occurs.

```rust,editable
{{#include ../../deps/examples/memmap.rs}}
```

{{#include ../refs/link-refs.md}}
