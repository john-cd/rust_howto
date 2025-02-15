# Current working directory

{{#include cwd.incl.md}}

## Get the current working directory {#cwd}

[![std][c-std-badge]][c-std]{{hi:std}} [![cat-filesystem][cat-filesystem-badge]][cat-filesystem]{{hi:Filesystem}}

```rust,editable
{{#include ../../../crates/cats/filesystem/tests/cwd/cwd.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[cwd: move to proper location (P1)](https://github.com/john-cd/rust_howto/issues/357)

- rename directory management
- find a spot for the following
- also cover std implementation

## `remove_dir_all` {#remove_dir_all}

[![remove_dir_all][c-remove_dir_all-badge]][c-remove_dir_all] [![remove_dir_all-crates.io][c-remove_dir_all-crates.io-badge]][c-remove_dir_all-crates.io] [![remove_dir_all-github][c-remove_dir_all-github-badge]][c-remove_dir_all-github] [![remove_dir_all-lib.rs][c-remove_dir_all-lib.rs-badge]][c-remove_dir_all-lib.rs]{{hi:remove_dir_all}}{{hi:Utility}}{{hi:Filesystem}}{{hi:Windows}}{{hi:Remove_dir}} [![cat-filesystem][cat-filesystem-badge]][cat-filesystem]{{hi:Filesystem}}

A safe, reliable implementation of [`remove_dir_all`][c-remove_dir_all]⮳{{hi:remove_dir_all}} for Windows

```rust,editable
{{#include ../../../crates/cats/filesystem/tests/remove_dir_all.rs:example}}
```

</div>
