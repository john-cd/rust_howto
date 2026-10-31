<!-- TODO Reviewed ??-??-2026 -->

# Directories

{{#include directories.incl.md}}

Use the following recipes to create, list, delete, and recursively traverse directories.

## Get the Current Working Directory {#cwd}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}} [![cat~filesystem][cat~filesystem~badge]][cat~filesystem]{{hi:Filesystem}}

`std::env::current_dir` returns the current working directory as a `Result<PathBuf>`:

```rust,editable
{{#include ../../../crates/cats/filesystem/examples/directories/cwd.rs:example}}
```

## Create, List Contents of, and Remove Directories {#create-list-contents-remove-dirs}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}} [![cat~filesystem][cat~filesystem~badge]][cat~filesystem]{{hi:Filesystem}}

The following example demonstrates basic operations on directories - creating & removing (recursively if needed), listing contents:

```rust,editable
{{#include ../../../crates/cats/filesystem/examples/directories/manipulate_dirs.rs:example}}
```

## Remove a Directory and its Contents with `remove_dir_all` {#remove_dir_all}

{{#badges remove_dir_all filesystem=Filesystem}}{{hi:Utility}}{{hi:Filesystem}}{{hi:Windows}}{{hi:Remove_dir}}

The [`remove_dir_all`][c~remove_dir_all~docs]↗{{hi:remove_dir_all}} library provides an alternative implementation of `std::fs::remove_dir_all` from the Rust `std` library.

In particular, its optional 'parallel' feature parallelizes the deletion. This is useful when high syscall latency is occurring, such as on Windows or network file systems.

```rust,editable,noplayground
{{#include ../../../crates/cats/filesystem/examples/directories/remove_dir_all.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[directory_traversal | Directory Traversal]].
- [[paths | Paths]].
- [[symbolic_links | Symbolic Links]].
- [[user_directories_and_preferred_applications | User Directories]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  accuracy review: file system
</div>
