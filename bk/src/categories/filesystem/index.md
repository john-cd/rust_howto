# File System

[![cat~filesystem][cat~filesystem~badge]][cat~filesystem]

This chapters deals with files{{hi:Files}} and file systems.{{hi:File systems}}

For most common file system tasks, [`std::fs`][c~std::fs~docs]↗{{hi:std::fs}} and [`std::path`][c~std::path~docs]↗{{hi:std::path}} will be sufficient. [`fs_extra`][c~fs_extra~docs]↗{{hi:fs_extra}} is useful for more advanced operations. [`tempfile`][c~tempfile~docs]↗{{hi:tempfile}} is great for working with temporary files. [`glob`][c~glob~docs]↗{{hi:glob}} simplifies file pattern matching. Use [`notify`][c~notify~docs]↗{{hi:notify}} for file watching.

| Topic | Rust Crates or Modules |
|---|---|
| File I/O | [`std::fs`][c~std::fs~docs]↗ in the Standard Library provides the core functionality for working with files and directories (opening, reading, writing, creating, deleting, etc.). |
| Paths | [`std::path`][c~std::path~docs]↗ provides types and functions for working with file paths ([`Path`][c~std::path::Path~docs]↗{{hi:Path}}, [`PathBuf`][c~std::path::PathBuf~docs]↗{{hi:PathBuf}}), extracting file names, extensions, and other path components. |
| Directories | [`std::fs`][c~std::fs~docs]↗ includes functions for creating, listing, and traversing directories. |
| File Metadata | [`std::fs`][c~std::fs~docs]↗ allows retrieving information about files (size, modification time, permissions, etc.). |
| File System Operations | [`fs_extra`][c~fs_extra~docs]↗{{hi:fs_extra}} provides additional file system operations, such as copying directories recursively, setting file permissions, etc. |
| Temporary Files and Directories | [`tempfile`][c~tempfile~docs]↗{{hi:tempfile}} creates temporary files and directories. |
| Globbing (Matching File Patterns) | [`glob`][c~glob~docs]↗{{hi:glob}} matches file paths using 'glob' patterns. |
| Symbolic Links | [`std::fs`][c~std::fs~docs]↗ supports working with symbolic links (though this is platform-dependent). |
| File Watching | [`notify`][c~notify~docs]↗{{hi:notify}} watches files and directories for changes. |

## Paths

{{#include paths.incl.md}}

## Reading and Writing Files

{{#include reading_and_writing_files.incl.md}}

### Working with the Standard Input and Output

{{#include working_with_the_standard_input_and_output.incl.md}}

### Symbolic Links

{{#include symbolic_links.incl.md}}

### File Metadata

{{#include file_metadata.incl.md}}

## Directories

{{#include directories.incl.md}}

### Directory Traversal

{{#include directory_traversal.incl.md}}

### User Directories and Preferred Applications

{{#include user_directories_and_preferred_applications.incl.md}}

## Temporary Files and Directories

{{#include temporary_files_and_directories.incl.md}}

## Globbing

FIXME.

## File Watching

{{#include file_watching.incl.md}}

## Related Topics

| Topic | Rust Crates or Modules |
|---|---|
| [[serde | Serialization/Deserialization]] | [`serde`][c~serde~docs]↗{{hi:serde}} is often used with file I/O to read and write structured data (JSON, YAML, TOML, etc.) to files. |
| [[compression | Compression/Decompression]] | [`flate2`][c~flate2~docs]↗{{hi:flate2}}, [`gzip`][c~gzip~docs]↗{{hi:gzip}}, [`bzip2`][c~bzip2~docs]↗{{hi:bzip2}}, [`xz2`][c~xz2~docs]↗{{hi:xz2}} work with various file compression formats. |
| [[tar | Archive Files]] | [`tar`][c~tar~docs]↗{{hi:tar}}, [`zip`][c~zip~docs]↗{{hi:zip}} can be used for working with `tar` and `zip` archives. |

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[review in depth](https://github.com/john-cd/rust_howto/issues/1339)

cover https://crates.io/crates/async-fs
</div>
