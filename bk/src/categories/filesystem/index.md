# File System

[![cat~filesystem][cat~filesystem~badge]][cat~filesystem]

Dealing with files{{hi:Files}} and file systems{{hi:File systems}}.

For most common file system tasks, [`std::fs`][c~std::fs~docs]↗{{hi:std::fs}} and [`std::path`][c~std::path~docs]↗{{hi:std::path}} will be sufficient. [`fs_extra`][c~fs_extra~docs]↗{{hi:fs_extra}} is useful for more advanced operations. [`tempfile`][c~tempfile~docs]↗{{hi:tempfile}} is great for working with temporary files. [`glob`][c~glob~docs]↗{{hi:glob}} simplifies file pattern matching. Use [`notify`][c~notify~docs]↗{{hi:notify}} for file watching.

| Topic | Rust Crates or Modules |
|---|---|
| File I/O | [`std::fs`][c~std::fs~docs]↗ (standard library) provides the core functionality for working with files and directories (opening, reading, writing, creating, deleting, etc.). |
| Paths | [`std::path`][c~std::path~docs]↗ (standard library) provides types and functions for working with file paths ([`Path`][c~std::path::Path~docs]↗{{hi:Path}}, [`PathBuf`][c~std::path::PathBuf~docs]↗{{hi:PathBuf}}), extracting file names, extensions, and other path components. |
| Directories | [`std::fs`][c~std::fs~docs]↗ includes functions for creating, listing, and traversing directories. |
| File Metadata | [`std::fs`][c~std::fs~docs]↗ allows you to retrieve information about files (size, modification time, permissions, etc.). |
| File System Operations | [`fs_extra`][c~fs_extra~docs]↗{{hi:fs_extra}} provides additional file system operations, such as copying directories recursively, setting file permissions, etc. |
| Temporary Files and Directories | [`tempfile`][c~tempfile~docs]↗{{hi:tempfile}} creates temporary files and directories. |
| Globbing (Matching File Patterns) | [`glob`][c~glob~docs]↗{{hi:glob}} matches file paths using 'glob' patterns. |
| Symbolic Links | [`std::fs`][c~std::fs~docs]↗ supports working with symbolic links (though this is platform-dependent). |
| File Watching | [`notify`][c~notify~docs]↗{{hi:notify}} watches files and directories for changes. |

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
| [[serde | Serialization/Deserialization]] | [`serde`][c~serde~docs]↗{{hi:serde}} is often used with file I/O to read and write structured data (JSON, YAML, TOML, etc.) to files. |
| [[compression | Compression/Decompression]] | [`flate2`][c~flate2~docs]↗{{hi:flate2}}, [`gzip`][c~gzip~docs]↗{{hi:gzip}}, [`bzip2`][c~bzip2~docs]↗{{hi:bzip2}}, [`xz2`][c~xz2~docs]↗{{hi:xz2}} work with various compression formats (often used when dealing with files). |
| [[tar | Archive Files]] (tar, zip, etc.) | [`tar`][c~tar~docs]↗{{hi:tar}}, [`zip`][c~zip~docs]↗{{hi:zip}} can be used for working with `tar` and `zip` archives. |

See also [[network-programming | Network Programming]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write, review in depth, add missing sections NOW](https://github.com/john-cd/rust_howto/issues/1339)

- [Rust Path vs PathBuf][blog~rust-path-vs-pathbuf]↗.

</div>
