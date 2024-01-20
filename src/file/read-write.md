# Read & Write

## Read lines of strings from a file

[![std-badge]][std] [![cat-filesystem-badge]][cat-filesystem]

Writes a three-line message to a file, then reads it back a line at a
time with the [`Lines`] iterator created by
[`BufRead::lines`]. [`File`] implements [`Read`] which provides [`BufReader`]
trait.  [`File::create`] opens a [`File`] for writing, [`File::open`] for
reading.

```rust,editable
{#include ../../../deps/examples/read-file.rs}
```

## Avoid writing and reading from a same file

[![same_file-badge]][same_file] [![cat-filesystem-badge]][cat-filesystem]

Use [`same_file::Handle`] to a file that can be tested for equality with
other handles. In this example, the handles of file to be read from and
to be written to are tested for equality.

```rust,editable,no_run
{#include ../../../deps/examples/same-file.rs}
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
than dealing with [`seek`] to navigate a File.

The [`Mmap::map`] function assumes the file
behind the memory map is not being modified at the same time by another process
or else a [race condition] occurs.

```rust,editable
{#include ../../../deps/examples/memmap.rs}
```

[`same_file::Handle`]: https://docs.rs/same-file/*/same_file/struct.Handle.html
[`BufRead::lines`]: https://doc.rust-lang.org/std/io/trait.BufRead.html#method.lines
[`BufRead`]: https://doc.rust-lang.org/std/io/trait.BufRead.html
[`BufReader`]: https://doc.rust-lang.org/std/io/struct.BufReader.html
[`File::create`]: https://doc.rust-lang.org/std/fs/struct.File.html#method.create
[`File::open`]: https://doc.rust-lang.org/std/fs/struct.File.html#method.open
[`File`]: https://doc.rust-lang.org/std/fs/struct.File.html
[`Lines`]: https://doc.rust-lang.org/std/io/struct.Lines.html
[`Read`]: https://doc.rust-lang.org/std/io/trait.Read.html
[`Mmap::map`]: https://docs.rs/memmap/*/memmap/struct.Mmap.html#method.map
[`seek`]: https://doc.rust-lang.org/std/fs/struct.File.html#method.seek
[race condition]: https://en.wikipedia.org/wiki/Race_condition#File_systems
{{#include ../refs/link-refs.md}}
