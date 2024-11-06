# Directory Traversal

{{#include dir.incl.md}}

## Find files that have been modified in the last 24 hours

[![std][c-std-badge]][c-std]{{hi:std}} [![cat-filesystem][cat-filesystem-badge]][cat-filesystem]{{hi:Filesystem}}

Gets the current working directory{{hi:Current working directory}} by calling [`std::env::current_dir`][c-std::env::current_dir]{{hi:std::env::current_dir}}⮳ then for each entries in [`std::fs::read_dir`][c-std::fs::read_dir]{{hi:std::fs::read_dir}}⮳ extracts the
[`std::fs::DirEntry::path`][c-std::fs::DirEntry::path]{{hi:std::fs::DirEntry::path}}⮳ and gets the metadata via [`std::fs::Metadata`][c-std::fs::Metadata]{{hi:std::fs::Metadata}}⮳. The
[`std::fs::Metadata::modified`][c-std::fs::Metadata::modified]{{hi:std::fs::Metadata::modified}}⮳ returns the [`std::time::SystemTime::elapsed`][c-std::time::SystemTime::elapsed]{{hi:std::time::SystemTime::elapsed}}⮳ time since last modification{{hi:Time since last modification}}. [`std::time::Duration::as_secs`][c-std::time::Duration::as_secs]{{hi:std::time::Duration::as_secs}}⮳ converts the time to seconds and compared with 24 hours (24 *60* 60 seconds). [`std::fs::Metadata::is_file`][c-std::fs::Metadata::is_file]{{hi:std::fs::Metadata::is_file}}⮳ filters out directories.

```rust
{{#include ../../../deps/tests/cats/filesystem/modified.rs:example}}
```

## Find loops for a given path

[![same-file][c-same_file-badge]][c-same_file]{{hi:same-file}}
[![same-file-crates.io][c-same_file-crates.io-badge]][c-same_file-crates.io]
[![same-file-github][c-same_file-github-badge]][c-same_file-github]
[![same-file-lib.rs][c-same_file-lib.rs-badge]][c-same_file-lib.rs]
[![cat-filesystem][cat-filesystem-badge]][cat-filesystem]{{hi:Filesystem}}

Use [`same_file::is_same_file`][c-same_file::is_same_file]{{hi:same_file::is_same_file}}⮳ to detect loops for a given path{{hi:Detect loops for a given path}}. For example, a loop could be created on a Unix system via symlinks:

```bash
mkdir -p /tmp/foo/bar/baz
ln -s /tmp/foo/ /tmp/foo/bar/baz/qux
```

The following would assert that a loop exists.

```rust
{{#include ../../../deps/tests/cats/filesystem/loops.rs:example}}
```

## Recursively find duplicate file names

[![cat-filesystem][cat-filesystem-badge]][cat-filesystem]{{hi:Filesystem}}

[![walkdir][c-walkdir-badge]][c-walkdir]{{hi:walkdir}}
[![walkdir-crates.io][c-walkdir-crates.io-badge]][c-walkdir-crates.io]
[![walkdir-github][c-walkdir-github-badge]][c-walkdir-github]
[![walkdir-lib.rs][c-walkdir-lib.rs-badge]][c-walkdir-lib.rs]

Find recursively in the current directory duplicate filenames{{hi:Duplicate filenames}}, printing them only once.

```rust
{{#include ../../../deps/tests/cats/filesystem/duplicate_name.rs:example}}
```

## Recursively find all files with given predicate

[![walkdir][c-walkdir-badge]][c-walkdir]{{hi:walkdir}}  [![cat-filesystem][cat-filesystem-badge]][cat-filesystem]{{hi:Filesystem}}

Find JSON files modified within the last day in the current directory. Using [`walkdir::WalkDir::follow_links`][c-walkdir::WalkDir::follow_links]{{hi:walkdir::WalkDir::follow_links}}⮳ ensures symbolic links{{hi:Symbolic links}} are followed like they were normal directories and files.

```rust
{{#include ../../../deps/tests/cats/filesystem/find_file.rs:example}}
```

## Traverse directories while skipping dotfiles

[![walkdir][c-walkdir-badge]][c-walkdir]{{hi:walkdir}}  [![cat-filesystem][cat-filesystem-badge]][cat-filesystem]{{hi:Filesystem}}{{hi:Skipping dotfiles}}

Uses [`walkdir::IntoIter::filter_entry`][c-walkdir::IntoIter::filter_entry]{{hi:walkdir::IntoIter::filter_entry}}⮳ to descend recursively into entries passing the `is_not_hidden` predicate thus skipping hidden files and directories. [`std::iter::Iterator::filter`][c-std::iter::Iterator::filter]{{hi:std::iter::Iterator::filter}}⮳ applies to each [`walkdir::IntoIter::filter_entry`][c-walkdir::IntoIter::filter_entry]{{hi:walkdir::IntoIter::filter_entry}}⮳ even if the parent is a hidden directory.

Root dir `"."` yields through `walkdir::WalkDir::depth` usage in `is_not_hidden` predicate.

```rust
{{#include ../../../deps/tests/cats/filesystem/skip_dot.rs:example}}
```

## Recursively calculate file sizes at given depth

[![walkdir][c-walkdir-badge]][c-walkdir]{{hi:walkdir}}  [![cat-filesystem][cat-filesystem-badge]][cat-filesystem]{{hi:Filesystem}}{{hi:File sizes}}

Recursion depth can be flexibly set by [`walkdir::Walkdir::min_depth`][c-walkdir::WalkDir::min_depth]{{hi:walkdir::WalkDir::min_depth}}⮳ & [`walkdir::WalkDir::max_depth`][c-walkdir::WalkDir::max_depth]{{hi:walkdir::WalkDir::max_depth}}⮳ methods. Calculates sum of all file sizes to 3 subfolders depth, ignoring files in the root folder.

```rust
{{#include ../../../deps/tests/cats/filesystem/sizes.rs:example}}
```

## Find all png files recursively

[![glob][c-glob-badge]][c-glob]{{hi:glob}}  [![cat-filesystem][cat-filesystem-badge]][cat-filesystem]{{hi:Filesystem}}

Recursively find all PNG files in the current directory. In this case, the `**` pattern matches the current directory and all subdirectories.

Use the `**` pattern in any path portion. For example, `/media/**/*.png` matches all PNGs in `media` and it's subdirectories.

```rust
{{#include ../../../deps/tests/cats/filesystem/png.rs:example}}
```

## Find all files with given pattern ignoring filename case

[![glob][c-glob-badge]][c-glob]{{hi:glob}}  [![cat-filesystem][cat-filesystem-badge]][cat-filesystem]{{hi:Filesystem}}

Find all image files in the `/media/` directory matching the `img_[0-9][0-9]*.png` pattern.

A custom [`glob::MatchOptions`][c-glob::MatchOptions]{{hi:glob::MatchOptions}}⮳ struct is passed to the [`glob::glob_with`][c-glob::glob_with]{{hi:glob::glob_with}}⮳ function making the glob{{hi:glob}} pattern case insensitive while keeping the other options [`std::default::Default`][c-std::default::Default]{{hi:std::default::Default}}⮳.

```rust
{{#include ../../../deps/tests/cats/filesystem/ignore_case.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO cleanup badges
TODO cover globset

[![globset][c-globset-badge]][c-globset]{{hi:globset}}
[![globset-crates.io][c-globset-crates.io-badge]][c-globset-crates.io]
[![globset-github][c-globset-github-badge]][c-globset-github]
[![globset-lib.rs][c-globset-lib.rs-badge]][c-globset-lib.rs]

High-performance globbing that allows multiple globs to be evaluated at once

</div>
