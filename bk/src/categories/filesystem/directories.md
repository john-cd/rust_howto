# Directories

{{#include directories.incl.md}}

Use the following recipes to create, list, delete, and recursively traverse directories.

## Get the Current Working Directory {#cwd}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}} [![cat~filesystem][cat~filesystem~badge]][cat~filesystem]{{hi:Filesystem}}

`std::env::current_dir` returns the current working directory as a `Result<PathBuf>`:

```rust,editable
{{#include ../../../crates/cats/filesystem/examples/directories/cwd.rs:example}}
```

## `remove_dir_all` {#remove_dir_all}

[![remove_dir_all][c~remove_dir_all~docs~badge]][c~remove_dir_all~docs] [![remove_dir_all~crates.io][c~remove_dir_all~crates.io~badge]][c~remove_dir_all~crates.io] [![remove_dir_all~github][c~remove_dir_all~github~badge]][c~remove_dir_all~github] [![remove_dir_all~lib.rs][c~remove_dir_all~lib.rs~badge]][c~remove_dir_all~lib.rs]{{hi:remove_dir_all}}{{hi:Utility}}{{hi:Filesystem}}{{hi:Windows}}{{hi:Remove_dir}} [![cat~filesystem][cat~filesystem~badge]][cat~filesystem]{{hi:Filesystem}}

FIXME A safe, reliable implementation of [`remove_dir_all`][c~remove_dir_all~docs]↗{{hi:remove_dir_all}} for Windows.

```rust,editable
{{#include ../../../crates/cats/filesystem/examples/directories/remove_dir_all.rs:example}}
```

also cover [`std`][c~std~docs]↗{{hi:std}} implementation

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[fix](https://github.com/john-cd/rust_howto/issues/357)
</div>
