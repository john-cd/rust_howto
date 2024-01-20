## Find all png files recursively

[![glob-badge]][glob] [![cat-filesystem-badge]][cat-filesystem]

Recursively find all PNG files in the current directory.
In this case, the `**` pattern matches the current directory and all subdirectories.

Use the `**` pattern in any path portion. For example, `/media/**/*.png`
matches all PNGs in `media` and it's subdirectories.

```rust,editable,no_run
{#include ../../../deps/examples/png.rs}
```
