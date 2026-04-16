# File Metadata

{{#include file_metadata.incl.md}}

## Display File Metadata {#metadata}

You can get file metadata in Rust using the `fs::metadata` function from the standard library, which returns a `Result<Metadata>`. The `std::fs::Metadata` struct contains information like file type, size, permissions, and modification times:

```rust,editable
{{#include ../../../crates/cats/filesystem/examples/metadata/metadata.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[read_write | Reading and Writing Files]].
- [[paths | Paths]].
- [[symbolic_links | Symbolic Links]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
</div>
