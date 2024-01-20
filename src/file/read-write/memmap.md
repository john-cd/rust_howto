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

[`Mmap::map`]: https://docs.rs/memmap/*/memmap/struct.Mmap.html#method.map
[`seek`]: https://doc.rust-lang.org/std/fs/struct.File.html#method.seek

[race condition]: https://en.wikipedia.org/wiki/Race_condition#File_systems
