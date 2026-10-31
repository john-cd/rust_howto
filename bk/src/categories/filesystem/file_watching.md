<!-- TODO Reviewed ??-??-2026 -->

# File Watching

{{#include file_watching.incl.md}}

## Watch Files or Directories and Execute a Function When They Change {#watch-files-or-dirs}

[![notify~website][c~notify~website~badge]][c~notify~website] {{#badges notify}} {{hi:Events}} {{hi:Filesystem}} {{hi:notify}} {{hi:Watch}} {{#categories filesystem=Filesystem}}

[`notify`][c~notify~docs]↗{{hi:notify}} is a cross-platform filesystem notification library.

This example demonstrates how to watch for file system events. It sets up a file system watcher that monitors a directory and its subdirectories for any changes, such as file creation, modification, or deletion:

```rust,editable,noplayground
{{#include ../../../crates/cats/filesystem/examples/file_watching/notify.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[directories | Directories]].
- [[directory_traversal | Directory Traversal]].
- [[paths | Paths]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  accuracy review: file system
</div>
