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

Reading and Writing Files: Basic file I/O operations, working with different file formats (CSV, JSON, TOML), and handling errors.
Working with Directories: Creating, listing, deleting, and recursively traversing directories.
Path Manipulation: Working with paths, extracting file names, extensions, and other path components.
Working with Standard Input/Output: Reading from stdin, writing to stdout/stderr, and handling command-line arguments.
Network I/O: Basic network programming with TCP and UDP sockets. Include examples of making HTTP requests and working with web servers.

</div>
