# Directory Traversal

{{#include dir.incl.md}}

## File names that have been modified in the last 24 hours

[![std][std-badge]][std]  [![cat-filesystem][cat-filesystem-badge]][cat-filesystem]

Gets the {{i:current working directory}} by calling [`env::current_dir`][std::env::current_dir] then for each entries in [`fs::read_dir`][std::fs::read_dir] extracts the
[`DirEntry::path`][std::fs::DirEntry::path] and gets the metadata via [`fs::Metadata`][std::fs::Metadata]. The
[`Metadata::modified`][std::fs::Metadata::modified] returns the [`SystemTime::elapsed`][std::time::SystemTime::elapsed] {{i:time since last modification}}. [`Duration::as_secs`][std::time::Duration::as_secs] converts the time to seconds and compared with 24 hours (24 *60* 60 seconds). [`Metadata::is_file`][std::fs::Metadata::is_file] filters out directories.

```rust,editable
{{#include ../../../deps/tests/modified.rs}}
```

## Find loops for a given path

[![same-file][same-file-badge]][same-file]  [![cat-filesystem][cat-filesystem-badge]][cat-filesystem]

Use [`same-file::is_same_file`][same-file::is_same_file] to {{i:detect loops for a given path}}. For example, a loop could be created on a Unix system via symlinks:

```bash
mkdir -p /tmp/foo/bar/baz
ln -s /tmp/foo/ /tmp/foo/bar/baz/qux
```

The following would assert that a loop exists.

```rust,editable,no_run
{{#include ../../../deps/tests/loops.rs}}
```

## Recursively find duplicate file names

[![walkdir][walkdir-badge]][walkdir]  [![cat-filesystem][cat-filesystem-badge]][cat-filesystem]

Find recursively in the current directory {{i:duplicate filenames}}, printing them only once.

```rust,editable,no_run
{{#include ../../../deps/tests/duplicate-name.rs}}
```

## Recursively find all files with given predicate

[![walkdir][walkdir-badge]][walkdir]  [![cat-filesystem][cat-filesystem-badge]][cat-filesystem]

Find JSON files modified within the last day in the current directory. Using [`follow_links`][walkdir::WalkDir::follow_links] ensures {{i:symbolic links}} are followed like they were normal directories and files.

```rust,editable,no_run
{{#include ../../../deps/tests/find-file.rs}}
```

## Traverse directories while {{i:skipping dotfiles}}

[![walkdir][walkdir-badge]][walkdir]  [![cat-filesystem][cat-filesystem-badge]][cat-filesystem]

Uses [`filter_entry`][walkdir::IntoIter::filter_entry] to descend recursively into entries passing the
`is_not_hidden` predicate thus skipping hidden files and directories.
 [`Iterator::filter`][std::iter::Iterator::filter] applies to each [`WalkDir::DirEntry`][walkdir::Walkdir::DirEntry] even if the parent
 is a hidden directory.

Root dir `"."` yields through [`WalkDir::depth`][walkdir::WalkDir::depth] usage in `is_not_hidden` predicate.

```rust,editable,no_run
{{#include ../../../deps/tests/skip-dot.rs}}
```

## Recursively calculate {{i:file sizes}} at given depth

[![walkdir][walkdir-badge]][walkdir]  [![cat-filesystem][cat-filesystem-badge]][cat-filesystem]

Recursion depth can be flexibly set by [`WalkDir::min_depth`][walkdir::Walkdir::min_depth] & [`WalkDir::max_depth`][walkdir::WalkDir::max_depth] methods. Calculates sum of all file sizes to 3 subfolders depth, ignoring files in the root folder.

```rust,editable
{{#include ../../../deps/tests/sizes.rs}}
```

## Find all png files recursively

[![glob][glob-badge]][glob]  [![cat-filesystem][cat-filesystem-badge]][cat-filesystem]

Recursively find all PNG files in the current directory. In this case, the `**` pattern matches the current directory and all subdirectories.

Use the `**` pattern in any path portion. For example, `/media/**/*.png` matches all PNGs in `media` and it's subdirectories.

```rust,editable,no_run
{{#include ../../../deps/tests/png.rs}}
```

## Find all files with given pattern ignoring filename case

[![glob][glob-badge]][glob]  [![cat-filesystem][cat-filesystem-badge]][cat-filesystem]

Find all image files in the `/media/` directory matching the `img_[0-9][0-9]*.png` pattern.

A custom [`MatchOptions`][glob::MatchOptions] struct is passed to the [`glob_with`][glob_with] function making the {{i:glob}} pattern case insensitive while keeping the other options [`Default`][std::default::Default]

```rust,editable,no_run
{{#include ../../../deps/tests/ignore-case.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
