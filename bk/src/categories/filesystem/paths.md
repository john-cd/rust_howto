# Paths

{{#include paths.incl.md}}

## Create and Manipulate Paths {#create-and-manipulate-paths}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}} [![cat~filesystem][cat~filesystem~badge]][cat~filesystem]{{hi:Filesystem}}

[`std::path`][c~std::path~docs]↗ provides types and functions for working with file paths. Its two main types are `Path` and `PathBuf`. The relationship between them is directly analogous to `str` and `String`:

- [`Path`][c~std::path::Path~docs]↗{{hi:Path}} is an unsized, immutable sequence of characters representing a file path. Because it is unsized, you almost always see it used as a borrowed slice: `&Path`. It is a "view" of a path string. It's efficient because it doesn't require a memory allocation.
- [`PathBuf`][c~std::path::PathBuf~docs]↗{{hi:PathBuf}} is an owned, mutable, growable buffer containing a file path. Use `PathBuf` when you need to build or modify a path.

Note that `Path` and `PathBuf` are thin wrappers around `OsString` and `OsStr` respectively, meaning that they work directly on strings according to the local platform's path syntax. Path methods that do not access the filesystem, such as `Path::starts_with` and `Path::ends_with`, are _case sensitive_ no matter the platform or filesystem. An exception to this is made for Windows drive letters.

The following example demonstrates the use of `Path` and `PathBuf`:

```rust,editable
{{#include ../../../crates/cats/filesystem/examples/paths/manipulate_paths.rs:example}}
```

## Canonicalize a Path {#canonicalize}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}} [![cat~filesystem][cat~filesystem~badge]][cat~filesystem]{{hi:Filesystem}}

The following returns the canonical, absolute form of a path with all intermediate components normalized and symbolic links resolved:

```rust,editable
{{#include ../../../crates/cats/filesystem/examples/paths/canonicalize.rs:example}}
```

## References {#references .skip}

- [Rust `Path` vs `PathBuf`][blog~rust-path-vs-pathbuf]↗.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
</div>
