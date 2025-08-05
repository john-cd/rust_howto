# Work with Directories

{{#include directories.incl.md}}

Creating, listing, deleting, and recursively traversing directories.

## Get the Current Working Directory {#cwd}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}} [![cat~filesystem][cat~filesystem~badge]][cat~filesystem]{{hi:Filesystem}}

```rust,editable
{{#include ../../../crates/cats/filesystem/examples/directories/cwd.rs:example}}
```

## `remove_dir_all` {#remove_dir_all}

[![remove_dir_all][c~remove_dir_all~docs~badge]][c~remove_dir_all~docs] [![remove_dir_all~crates.io][c~remove_dir_all~crates.io~badge]][c~remove_dir_all~crates.io] [![remove_dir_all~github][c~remove_dir_all~github~badge]][c~remove_dir_all~github] [![remove_dir_all~lib.rs][c~remove_dir_all~lib.rs~badge]][c~remove_dir_all~lib.rs]{{hi:remove_dir_all}}{{hi:Utility}}{{hi:Filesystem}}{{hi:Windows}}{{hi:Remove_dir}} [![cat~filesystem][cat~filesystem~badge]][cat~filesystem]{{hi:Filesystem}}

A safe, reliable implementation of [`remove_dir_all`][c~remove_dir_all~docs]↗{{hi:remove_dir_all}} for Windows.

```rust,editable
{{#include ../../../crates/cats/filesystem/examples/directories/remove_dir_all.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[fix - also cover [`std`][c~std~docs]↗{{hi:std}} implementation. NOW](https://github.com/john-cd/rust_howto/issues/357)
</div>
