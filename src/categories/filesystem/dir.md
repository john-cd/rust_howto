# Directory Traversal

{{#include dir.incl.md}}

## File names that have been modified in the last 24 hours

[![std][c-std-badge]][c-std]  [![cat-filesystem][cat-filesystem-badge]][cat-filesystem]

Gets the {{i:current working directory}} by calling {{hi:env::current_dir}}[`env::current_dir`][c-std::env::current_dir]⮳ then for each entries in {{hi:fs::read_dir}}[`fs::read_dir`][c-std::fs::read_dir]⮳ extracts the
{{hi:DirEntry::path}}[`DirEntry::path`][c-std::fs::DirEntry::path]⮳ and gets the metadata via {{hi:fs::Metadata}}[`fs::Metadata`][c-std::fs::Metadata]⮳. The
{{hi:Metadata::modified}}[`Metadata::modified`][c-std::fs::Metadata::modified]⮳ returns the {{hi:SystemTime::elapsed}}[`SystemTime::elapsed`][c-std::time::SystemTime::elapsed]⮳ {{i:time since last modification}}. {{hi:Duration::as_secs}}[`Duration::as_secs`][c-std::time::Duration::as_secs]⮳ converts the time to seconds and compared with 24 hours (24 *60* 60 seconds). {{hi:Metadata::is_file}}[`Metadata::is_file`][c-std::fs::Metadata::is_file]⮳ filters out directories.

```rust,editable
{{#include ../../../deps/tests/modified.rs}}
```

## Find loops for a given path

[![same-file][c-same-file-badge]][c-same-file]  [![cat-filesystem][cat-filesystem-badge]][cat-filesystem]

Use {{hi:same-file::is_same_file}}[`same-file::is_same_file`][c-same-file::is_same_file]⮳ to {{i:detect loops for a given path}}. For example, a loop could be created on a Unix system via symlinks:

```bash
mkdir -p /tmp/foo/bar/baz
ln -s /tmp/foo/ /tmp/foo/bar/baz/qux
```

The following would assert that a loop exists.

```rust,editable,no_run
{{#include ../../../deps/tests/loops.rs}}
```

## Recursively find duplicate file names

[![walkdir][c-walkdir-badge]][c-walkdir]  [![cat-filesystem][cat-filesystem-badge]][cat-filesystem]

Find recursively in the current directory {{i:duplicate filenames}}, printing them only once.

```rust,editable,no_run
{{#include ../../../deps/tests/duplicate-name.rs}}
```

## Recursively find all files with given predicate

[![walkdir][c-walkdir-badge]][c-walkdir]  [![cat-filesystem][cat-filesystem-badge]][cat-filesystem]

Find JSON files modified within the last day in the current directory. Using {{hi:follow_links}}[`follow_links`][c-walkdir::WalkDir::follow_links]⮳ ensures {{i:symbolic links}} are followed like they were normal directories and files.

```rust,editable,no_run
{{#include ../../../deps/tests/find-file.rs}}
```

## Traverse directories while skipping dotfiles

[![walkdir][c-walkdir-badge]][c-walkdir]  [![cat-filesystem][cat-filesystem-badge]][cat-filesystem] {{hi:skipping dotfiles}}

Uses {{hi:filter_entry}}[`filter_entry`][c-walkdir::IntoIter::filter_entry]⮳ to descend recursively into entries passing the `is_not_hidden` predicate thus skipping hidden files and directories. {{hi:Iterator::filter}}[`Iterator::filter`][c-std::iter::Iterator::filter]⮳ applies to each {{hi:WalkDir::DirEntry}}[`WalkDir::DirEntry`][c-walkdir::IntoIter::filter_entry]⮳ even if the parent is a hidden directory.

Root dir `"."` yields through {{hi:WalkDir::depth}}[`WalkDir::depth`][c-walkdir::WalkDir::depth]⮳ usage in `is_not_hidden` predicate.

```rust,editable,no_run
{{#include ../../../deps/tests/skip-dot.rs}}
```

## Recursively calculate file sizes at given depth

[![walkdir][c-walkdir-badge]][c-walkdir]  [![cat-filesystem][cat-filesystem-badge]][cat-filesystem] {{hi:file sizes}}

Recursion depth can be flexibly set by {{hi:WalkDir::min_depth}}[`WalkDir::min_depth`][c-walkdir::Walkdir::min_depth]⮳ & {{hi:WalkDir::max_depth}}[`WalkDir::max_depth`][c-walkdir::WalkDir::max_depth]⮳ methods. Calculates sum of all file sizes to 3 subfolders depth, ignoring files in the root folder.

```rust,editable
{{#include ../../../deps/tests/sizes.rs}}
```

## Find all png files recursively

[![glob][c-glob-badge]][c-glob]  [![cat-filesystem][cat-filesystem-badge]][cat-filesystem]

Recursively find all PNG files in the current directory. In this case, the `**` pattern matches the current directory and all subdirectories.

Use the `**` pattern in any path portion. For example, `/media/**/*.png` matches all PNGs in `media` and it's subdirectories.

```rust,editable,no_run
{{#include ../../../deps/tests/png.rs}}
```

## Find all files with given pattern ignoring filename case

[![glob][c-glob-badge]][c-glob]  [![cat-filesystem][cat-filesystem-badge]][cat-filesystem]

Find all image files in the `/media/` directory matching the `img_[0-9][0-9]*.png` pattern.

A custom {{hi:MatchOptions}}[`MatchOptions`][c-glob::MatchOptions]⮳ struct is passed to the {{hi:glob_with}}[`glob_with`][c-glob::glob_with]⮳ function making the {{i:glob}} pattern case insensitive while keeping the other options {{hi:Default}}[`Default`][c-std::default::Default]⮳.

```rust,editable,no_run
{{#include ../../../deps/tests/ignore-case.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
