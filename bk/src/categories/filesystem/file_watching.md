# File Watching

{{#include file_watching.incl.md}}

## Watch Files or Directories and Execute a Function When They Change {#watch-files-or-dirs}

[![notify~website][c~notify~website~badge]][c~notify~website] [![notify][c~notify~docs~badge]][c~notify~docs] [![notify~crates.io][c~notify~crates.io~badge]][c~notify~crates.io] [![notify~repo][c~notify~repo~badge]][c~notify~repo] [![notify~lib.rs][c~notify~lib.rs~badge]][c~notify~lib.rs]{{hi:notify}}{{hi:Events}}{{hi:Filesystem}}{{hi:notify}}{{hi:Watch}} [![cat~filesystem][cat~filesystem~badge]][cat~filesystem]{{hi:Filesystem}}

[`notify`][c~notify~docs]↗{{hi:notify}} is a cross-platform filesystem notification library.

This example demonstrates how to watch for file system events. It sets up a file system watcher that monitors a directory and its subdirectories for any changes, such as file creation, modification, or deletion:

```rust,editable
{{#include ../../../crates/cats/filesystem/examples/file_watching/notify.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[directories | Directories]].
- [[directory_traversal | Directory Traversal]].
- [[paths | Paths]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
</div>
