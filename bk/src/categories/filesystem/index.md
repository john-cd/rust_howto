# File System

[![cat-filesystem][cat-filesystem-badge]][cat-filesystem]

Dealing with files{{hi:Files}} and file systems{{hi:File systems}}.

For most common file system tasks, `std::fs` and `std::path` will be sufficient. [`fs_extra`][c-fs_extra]⮳{{hi:fs_extra}} is useful for more advanced operations. [`tempfile`][c-tempfile]⮳{{hi:tempfile}} is great for working with temporary files. [`glob`][c-glob]⮳{{hi:glob}} simplifies file pattern matching. Use [`notify`][c-notify]⮳{{hi:notify}} for file watching.

| Topic | Rust Crates or Modules |
|---|---|
| File I/O | `std::fs` (standard library) provides the core functionality for working with files and directories (opening, reading, writing, creating, deleting, etc.). |
| Paths | `std::path` (standard library) provides types and functions for working with file paths (`Path`, `PathBuf`), extracting file names, extensions, and other path components. |
| Directories | `std::fs` (standard library) includes functions for creating, listing, and traversing directories. |
| File Metadata | `std::fs` (standard library) allows you to retrieve information about files (size, modification time, permissions, etc.). |
| File System Operations | [`fs_extra`][c-fs_extra]⮳{{hi:fs_extra}} provides additional file system operations, such as copying directories recursively, setting file permissions, etc. |
| Temporary Files and Directories | [`tempfile`][c-tempfile]⮳{{hi:tempfile}} creates temporary files and directories. |
| Globbing (Matching File Patterns) | [`glob`][c-glob]⮳{{hi:glob}} matches file paths using 'glob' patterns. |
| Symbolic Links | `std::fs` (Standard library) supports working with symbolic links (though this is platform-dependent). |
| File Watching | [`notify`][c-notify]⮳{{hi:notify}} watches files and directories for changes. |

## Paths

FIXME

## Reading & Writing Files

{{#include read-write.incl.md}}

### Working with Standard Input/Output

Reading from stdin, writing to stdout/stderr, and handling command-line arguments.

FIXME

### Symbolic Links

FIXME

### File Metadata

FIXME

## Directories

{{#include directories.incl.md}}

### Directory Traversal

{{#include directory_traversal.incl.md}}

### Walk the Filesystem While Respecting Ignore Files

{{#include _ignore.incl.md}}

### User Directories

{{#include user_directories.incl.md}}

## Temporary Files and Directories

{{#include tempfile.incl.md}}

## Globbing

FIXME

## File Watching

{{#include file_watching.incl.md}}

## Related Topics

| Topic | Rust Crates or Modules |
|---|---|
| [[serde | Serialization/Deserialization]] | [`serde`][c-serde]⮳{{hi:serde}} is often used with file I/O to read and write structured data (JSON, YAML, TOML, etc.) to files. |
| [[compression | Compression/Decompression]] | [`flate2`][c-flate2]⮳{{hi:flate2}}, [`gzip`][c-gzip]⮳{{hi:gzip}}, [`bzip2`][c-bzip2]⮳{{hi:bzip2}}, [`xz2`][c-xz2]⮳{{hi:xz2}} work with various compression formats (often used when dealing with files). |
| [[tar | Archive Files]] (tar, zip, etc.) | [`tar`][c-tar]⮳{{hi:tar}}, [`zip`][c-zip]⮳{{hi:zip}} can be used for working with tar and zip archives. |

See also [[network-programming | Network Programming]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO write, review in depth, add missing sections NOW

| [`open`][ex-filesystem-open] | [![open][c-open-badge]][c-open] | [![cat-filesystem][cat-filesystem-badge]][cat-filesystem] |

## `open` {#open}

[![open][c-open-badge]][c-open] [![open-crates.io][c-open-crates.io-badge]][c-open-crates.io] [![open-github][c-open-github-badge]][c-open-github] [![open-lib.rs][c-open-lib.rs-badge]][c-open-lib.rs]{{hi:open}}{{hi:open}}{{hi:Xdg-open}}{{hi:Start}}{{hi:Launch}}

[`open`][c-open]⮳{{hi:open}} opens a path or [URL][p-url] using the program configured on the system.

```rust,editable
{{#include ../../../crates/cats/filesystem/tests/open.rs:example}}
```

</div>
