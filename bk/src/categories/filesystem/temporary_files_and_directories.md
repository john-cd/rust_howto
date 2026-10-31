<!-- TODO Reviewed ??-??-2026 -->

# Temporary Files and Directories

{{#include temporary_files_and_directories.incl.md}}

## Create Temporary Files or Temporary Directories {#temporary-files-or-directories}

[![tempfile~website][c~tempfile~website~badge]][c~tempfile~website] {{#badges tempfile}} {{hi:Filesystem}} {{hi:Tempfile}}

The following example demonstrates the use of the [`tempfile`][c~tempfile~docs]↗ crate for creating temporary files and directories that are automatically deleted when no longer referenced.

Note that temporary files and directories have subtle security and resource leaking risks. Caveat lector!

```rust,editable
{{#include ../../../crates/cats/filesystem/examples/tempfile/tempfile.rs:example}}
```

### Other Options {#other-options .skip}

- The standalone `tempdir` crate is deprecated; use [`tempfile::tempdir`][c~tempfile::tempdir~docs]↗{{hi:tempfile::tempdir}} from [`tempfile`][c~tempfile~docs]↗{{hi:tempfile}} instead.

## Related Topics {#related-topics .skip}

- [[reading_and_writing_files | Reading and Writing Files]].
- [[paths | Paths]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  accuracy review: file system
</div>
