# Temporary Files and Directories

{{#include tempfile.incl.md}}

## Create Temporary Files or Temporary Directories {#temporary-files-or-directories}

[![tempfile~website][c~tempfile~website~badge]][c~tempfile~website] [![tempfile][c~tempfile~docs~badge]][c~tempfile~docs] [![tempfile~crates.io][c~tempfile~crates.io~badge]][c~tempfile~crates.io] [![tempfile~github][c~tempfile~github~badge]][c~tempfile~github] [![tempfile~lib.rs][c~tempfile~lib.rs~badge]][c~tempfile~lib.rs]{{hi:tempfile}}{{hi:Filesystem}}{{hi:Tempfile}}

The following example demonstrates the use of the [`tempfile`][c~tempfile~docs]↗ crate for creating temporary files and directories that are automatically deleted when no longer referenced.

Note that temporary files and directories have subtle security and resource leaking risks. Caveat lector!

```rust,editable
{{#include ../../../crates/cats/filesystem/examples/tempfile/tempfile.rs:example}}
```

### Other Options {#skip}

- The [`tempdir`][c~tempdir~docs]↗{{hi:tempdir}} crate is being merged into [`tempfile`][c~tempfile~docs]↗{{hi:tempfile}}.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
</div>
