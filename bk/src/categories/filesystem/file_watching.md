<!-- TODO Reviewed ??-??-2026. -->

# File Watching

{{#include file_watching.incl.md}}

## Watch Files or Directories and Execute a Function When They Change {#watch-files-or-dirs}

[![notify~website][c~notify~website~badge]][c~notify~website] {{#badges notify}} {{hi:Events}} {{hi:Filesystem}} {{hi:notify}} {{hi:Watch}} {{#categories filesystem=Filesystem}}

[`notify`][c~notify~docs]↗{{hi:notify}} is a cross-platform filesystem notification library.

This example demonstrates how to watch for file system events. It sets up a file system watcher that monitors a directory and its subdirectories for any changes, such as file creation, modification, or deletion:

```rust,editable,noplayground
{{#include ../../../crates/cats/filesystem/examples/file_watching/notify.rs:example}}
```

## Watch an Editor-Style File Replacement {#debounced-save}

Use `notify-debouncer-full` to receive batches after a short delay. Watch the containing directory because editors often write to a staging file and replace the destination. Check the paths in the delivered batch and use a deadline so the test reports a missing notification.

```rust,ignore,noplayground
{{#include ../../../crates/cats/filesystem/examples/debounced_save.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[directories | Directories]].
- [[directory_traversal | Directory Traversal]].
- [[paths | Paths]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
