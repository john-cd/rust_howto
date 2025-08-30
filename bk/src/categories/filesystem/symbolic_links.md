# Symbolic Links

{{#include symbolic_links.incl.md}}

## Work with Symbolic Links {#symbolic-links}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}} [![cat~filesystem][cat~filesystem~badge]][cat~filesystem]{{hi:Filesystem}}

Symbolic links are handled primarily through the standard library's `std::fs` module, with platform-specific extensions in `std::os::unix::fs` and `std::os::windows::fs`. These modules provide functions to create, read, and inspect symbolic links without following them to their target:

```rust,editable
{{#include ../../../crates/cats/filesystem/examples/symbolic_links/symbolic_links.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
</div>
