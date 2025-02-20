# File System

[![cat-filesystem][cat-filesystem-badge]][cat-filesystem]

Dealing with files{{hi:Files}} and file systems{{hi:File systems}}.

## File Reading & Writing

{{#include read-write.incl.md}}

## Current Working Directory

{{#include cwd.incl.md}}

## User Directories

{{#include user_directories.incl.md}}

## Temporary Files and Directories

{{#include tempfile.incl.md}}

## Directory Traversal

{{#include directory_traversal.incl.md}}

## Walk the Filesystem while Respecting Ignore Files

{{#include _ignore.incl.md}}

## File Watching

{{#include file_watching.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">

Working with the File System and I/O:

- Reading and Writing Files: Basic file I/O operations, working with different file formats (CSV, JSON, TOML), and handling errors.
- Working with Directories: Creating, listing, deleting, and recursively traversing directories.
- Path Manipulation: Working with paths, extracting file names, extensions, and other path components.
- Working with Standard Input/Output: Reading from stdin, writing to stdout/stderr, and handling command-line arguments.
- Network I/O: Basic network programming with TCP and UDP sockets. Include examples of making HTTP requests and working with web servers.

## File I/O

`std::fs` (Standard library) Provides the core functionality for working with files and directories (opening, reading, writing, creating, deleting, etc.).

## Paths

`std::path`: (Standard library) Provides types and functions for working with file paths (Path, PathBuf).

## Working with Directories

`std::fs`: (Standard library) Includes functions for creating, listing, and traversing directories.

## File Metadata

`std::fs`: (Standard library) Allows you to retrieve information about files (size, modification time, permissions, etc.).

## File System Operations (More Advanced)

`fs_extra`: A crate that provides additional file system operations, such as copying directories recursively, setting file permissions, etc.

## Temporary Files and Directories 2

`tempfile`: A crate for creating temporary files and directories.

## Globbing (Matching File Patterns)

`glob`: A crate for matching file paths using glob patterns.

## Symbolic Links

`std::fs`: (Standard library) Supports working with symbolic links (though this is platform-dependent).

## File Watching

`notify`: A crate for watching files and directories for changes.

## Serialization/Deserialization (for file content)

`serde`: Not a file system crate itself, but extremely relevant

Often used with file I/O to read and write structured data (JSON, YAML, TOML, etc.) to files.

## Compression/Decompression

`flate2`, `gzip`, `bzip2`, `xz2`: Crates for working with various compression formats (often used when dealing with files).

## Archive Files (tar, zip, etc.)

`tar`, `zip`: Crates for working with tar and zip archives.

For most common file system tasks, `std::fs` and `std::path` will be sufficient. `fs_extra` is useful for more advanced operations. `tempfile` is great for working with temporary files.  `glob` simplifies file pattern matching.  `notify` is essential for file watching.  And, of course, `serde` is very often used to handle the contents of files when you're working with structured data.

</div>
