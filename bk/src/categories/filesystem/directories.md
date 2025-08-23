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

The following example demonstrates basic operations on directories - creating & removing (recursively if needed), list contents:

```rust,editable
{{#include ../../../crates/cats/filesystem/examples/directories/manipulate_dirs.rs:example}}
```

## Remove a Directory and its Contents with `remove_dir_all` {#remove_dir_all}

[![remove_dir_all][c~remove_dir_all~docs~badge]][c~remove_dir_all~docs] [![remove_dir_all~crates.io][c~remove_dir_all~crates.io~badge]][c~remove_dir_all~crates.io] [![remove_dir_all~repo][c~remove_dir_all~repo~badge]][c~remove_dir_all~repo] [![remove_dir_all~lib.rs][c~remove_dir_all~lib.rs~badge]][c~remove_dir_all~lib.rs]{{hi:remove_dir_all}}{{hi:Utility}}{{hi:Filesystem}}{{hi:Windows}}{{hi:Remove_dir}} [![cat~filesystem][cat~filesystem~badge]][cat~filesystem]{{hi:Filesystem}}

The [`remove_dir_all`][c~remove_dir_all~docs]↗{{hi:remove_dir_all}} library provides an alternative implementation of `std::fs::remove_dir_all` from the Rust `std` library.

In particular, its optional 'parallel' feature parallelizes the deletion. This is useful when high syscall latency is occurring, such as on Windows or network file systems.

```rust,editable
{{#include ../../../crates/cats/filesystem/examples/directories/remove_dir_all2.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
</div>
