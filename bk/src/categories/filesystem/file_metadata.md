<!-- TODO Reviewed ??-??-2026 -->

# File Metadata

{{#include file_metadata.incl.md}}

## Display File Metadata {#metadata}

You can get file metadata in Rust using the `fs::metadata` function from the standard library, which returns a `Result<Metadata>`. The `std::fs::Metadata` struct contains information like file type, size, permissions, and modification times:

```rust,editable
{{#include ../../../crates/cats/filesystem/examples/metadata/metadata.rs:example}}
```

## Detect a File's Type from its Content {#infer}

[![infer][c~infer~docs~badge]][c~infer~docs]{{hi:infer}} [![infer~crates.io][c~infer~crates.io~badge]][c~infer~crates.io] [![infer~repo][c~infer~repo~badge]][c~infer~repo] [![infer~lib.rs][c~infer~lib.rs~badge]][c~infer~lib.rs] [![cat~filesystem][cat~filesystem~badge]][cat~filesystem]{{hi:Filesystem}}

A file extension can be missing or wrong. [`infer`][c~infer~docs]↗{{hi:infer}} inspects the first bytes of a file (its "magic number") and returns its type and MIME type. `infer::get` works on bytes in memory, `infer::get_from_path` reads the start of a file, and helpers like `infer::is_image` match a whole family of types. Content that has no magic number, such as plain text, is not recognized.

```rust,editable
{{#include ../../../crates/cats/filesystem/examples/metadata/infer.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[reading_and_writing_files | Reading and Writing Files]].
- [[paths | Paths]].
- [[symbolic_links | Symbolic Links]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  accuracy review: file system
</div>
