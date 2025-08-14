# Directory Traversal

{{#include directory_traversal.incl.md}}

## Find Files Modified in the Last 24 Hours {#find-files-modified-last-24-hours}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}} [![cat~filesystem][cat~filesystem~badge]][cat~filesystem]{{hi:Filesystem}}

Gets the current working directory{{hi:Current working directory}} by calling [`std::env::current_dir`][c~std::env::current_dir~docs]↗{{hi:std::env::current_dir}} then for each entries in [`std::fs::read_dir`][c~std::fs::read_dir~docs]↗{{hi:std::fs::read_dir}} extracts the [`std::fs::DirEntry::path`][c~std::fs::DirEntry::path~docs]↗{{hi:std::fs::DirEntry::path}} and gets the metadata via [`std::fs::Metadata`][c~std::fs::Metadata~docs]↗{{hi:std::fs::Metadata}}. The [`std::fs::Metadata::modified`][c~std::fs::Metadata::modified~docs]↗{{hi:std::fs::Metadata::modified}} returns the [`std::time::SystemTime::elapsed`][c~std::time::SystemTime::elapsed~docs]↗{{hi:std::time::SystemTime::elapsed}} time since last modification{{hi:Time since last modification}}. [`std::time::Duration::as_secs`][c~std::time::Duration::as_secs~docs]↗{{hi:std::time::Duration::as_secs}} converts the time to seconds and compared with 24 hours (24 *60* 60 seconds). [`std::fs::Metadata::is_file`][c~std::fs::Metadata::is_file~docs]↗{{hi:std::fs::Metadata::is_file}} filters out directories.

```rust,editable
{{#include ../../../crates/cats/filesystem/examples/directory_traversal/modified.rs:example}}
```

## Find Loops in a Given Path {#find-loop-in-path}

[![same-file][c~same-file~docs~badge]][c~same-file~docs] [![same-file~crates.io][c~same-file~crates.io~badge]][c~same-file~crates.io] [![same-file~github][c~same-file~github~badge]][c~same-file~github] [![same-file~lib.rs][c~same-file~lib.rs~badge]][c~same-file~lib.rs]{{hi:same-file}}{{hi:Same}}{{hi:Equal}}{{hi:Inode}}{{hi:File}}

[`same-file`][c~same-file~docs]↗{{hi:same-file}} is a simple crate for determining whether two file paths point to the same file.

Use [`same-file::is_same-file`][c~same-file::is_same-file~docs]↗{{hi:same-file::is_same-file}} to detect loops for a given path{{hi:Detect loops for a given path}}. For example, a loop could be created on a Unix system via symlinks:

```bash
mkdir -p /tmp/foo/bar/baz
ln -s /tmp/foo/ /tmp/foo/bar/baz/qux
```

The following would assert that a loop exists.

```rust,editable
{{#include ../../../crates/cats/filesystem/examples/directory_traversal/loops.rs:example}}
```

## Recursively Find Duplicate File Names {#recursively-find-duplicate-file-names}

[![walkdir][c~walkdir~docs~badge]][c~walkdir~docs] [![walkdir~crates.io][c~walkdir~crates.io~badge]][c~walkdir~crates.io] [![walkdir~github][c~walkdir~github~badge]][c~walkdir~github] [![walkdir~lib.rs][c~walkdir~lib.rs~badge]][c~walkdir~lib.rs]{{hi:walkdir}}{{hi:Directory}}{{hi:Walk}}{{hi:Recursive}}{{hi:Iterator}} [![cat~filesystem][cat~filesystem~badge]][cat~filesystem]{{hi:Filesystem}}

Find recursively in the current directory duplicate filenames{{hi:Duplicate filenames}}, printing them only once.

```rust,editable
{{#include ../../../crates/cats/filesystem/examples/directory_traversal/duplicate_name.rs:example}}
```

## Recursively Find All Files with a Given Predicate {#recursively-find-all-files-with-given-predicate}

[![walkdir][c~walkdir~docs~badge]][c~walkdir~docs] [![walkdir~crates.io][c~walkdir~crates.io~badge]][c~walkdir~crates.io] [![walkdir~github][c~walkdir~github~badge]][c~walkdir~github] [![walkdir~lib.rs][c~walkdir~lib.rs~badge]][c~walkdir~lib.rs]{{hi:walkdir}}{{hi:Directory}}{{hi:Walk}}{{hi:Recursive}}{{hi:Iterator}} [![cat~filesystem][cat~filesystem~badge]][cat~filesystem]{{hi:Filesystem}}

Find files modified within the last day in the current directory. Using [`walkdir::WalkDir::follow_links`][c~walkdir::WalkDir::follow_links~docs]↗{{hi:walkdir::WalkDir::follow_links}} ensures symbolic links{{hi:Symbolic links}} are followed like they were normal directories and files.

```rust,editable
{{#include ../../../crates/cats/filesystem/examples/directory_traversal/find_file.rs:example}}
```

## Traverse Directories While Skipping Dotfiles {#traverse-directories-while-skipping-dotfiles}

[![walkdir][c~walkdir~docs~badge]][c~walkdir~docs] [![walkdir~crates.io][c~walkdir~crates.io~badge]][c~walkdir~crates.io] [![walkdir~github][c~walkdir~github~badge]][c~walkdir~github] [![walkdir~lib.rs][c~walkdir~lib.rs~badge]][c~walkdir~lib.rs]{{hi:walkdir}}{{hi:Directory}}{{hi:Walk}}{{hi:Recursive}}{{hi:Iterator}} [![cat~filesystem][cat~filesystem~badge]][cat~filesystem]{{hi:Filesystem}}

Uses [`walkdir::IntoIter::filter_entry`][c~walkdir::IntoIter::filter_entry~docs]↗{{hi:walkdir::IntoIter::filter_entry}} to descend recursively into entries passing the `is_not_hidden` predicate thus skipping hidden files and directories. [`std::iter::Iterator::filter`][c~std::iter::Iterator::filter~docs]↗{{hi:std::iter::Iterator::filter}} applies to each [`walkdir::IntoIter::filter_entry`][c~walkdir::IntoIter::filter_entry~docs]↗{{hi:walkdir::IntoIter::filter_entry}} even if the parent is a hidden directory.

Root dir `"."` yields through [`walkdir::WalkDir::depth`][c~walkdir::WalkDir::depth~docs]↗{{hi:walkdir::WalkDir::depth}} usage in `is_not_hidden` predicate.

```rust,editable
{{#include ../../../crates/cats/filesystem/examples/directory_traversal/skip_dot.rs:example}}
```

## Walk the Filesystem While Respecting Ignore Files {#walk-the-filesystem-while-respecting-ignore-files}

[![ignore][c~ignore~docs~badge]][c~ignore~docs] [![ignore~crates.io][c~ignore~crates.io~badge]][c~ignore~crates.io] [![ignore~github][c~ignore~github~badge]][c~ignore~github] [![ignore~lib.rs][c~ignore~lib.rs~badge]][c~ignore~lib.rs]{{hi:ignore}}{{hi:File}}{{hi:Gitignore}}{{hi:Glob}}{{hi:ignore}}{{hi:Pattern}}

[`ignore`][c~ignore~docs]↗{{hi:ignore}} [`ignore`][c~serde_ignored~docs]↗{{hi:ignore}} is a library for efficiently matching ignore files such as [`.gitignore`][git-gitignore~website]↗{{hi:.gitignore}} against file paths.

Recursive [filesystem][p~filesystem] walking that respects ignore files (like .gitignore)

```rust,editable
{{#include ../../../crates/cats/filesystem/examples/directory_traversal/ignore.rs:example}}
```

## Recursively Calculate File Sizes at a Given Depth {#recursively-calculate-file-sizes-at-given-depth}

[![walkdir][c~walkdir~docs~badge]][c~walkdir~docs] [![walkdir~crates.io][c~walkdir~crates.io~badge]][c~walkdir~crates.io] [![walkdir~github][c~walkdir~github~badge]][c~walkdir~github] [![walkdir~lib.rs][c~walkdir~lib.rs~badge]][c~walkdir~lib.rs]{{hi:walkdir}}{{hi:Directory}}{{hi:Walk}}{{hi:Recursive}}{{hi:Iterator}} [![cat~filesystem][cat~filesystem~badge]][cat~filesystem]{{hi:Filesystem}}{{hi:File sizes}}

Recursion depth can be flexibly set by [`walkdir::Walkdir::min_depth`][c~walkdir::WalkDir::min_depth~docs]↗{{hi:walkdir::WalkDir::min_depth}} & [`walkdir::WalkDir::max_depth`][c~walkdir::WalkDir::max_depth~docs]↗{{hi:walkdir::WalkDir::max_depth}} methods. Calculates sum of all file sizes to 3 subfolders depth, ignoring files in the root folder.

```rust,editable
{{#include ../../../crates/cats/filesystem/examples/directory_traversal/sizes.rs:example}}
```

## Find All Files with a Given Extension Recursively {#find-all-png-files-recursively}

[![glob][c~glob~docs~badge]][c~glob~docs] [![glob~crates.io][c~glob~crates.io~badge]][c~glob~crates.io] [![glob~github][c~glob~github~badge]][c~glob~github] [![glob~lib.rs][c~glob~lib.rs~badge]][c~glob~lib.rs]{{hi:glob}} [![cat~filesystem][cat~filesystem~badge]][cat~filesystem]{{hi:Filesystem}}

Recursively find all PNG files in the current directory. In this case, the `**` pattern matches the current directory and all subdirectories.

Use the `**` pattern in any path portion. For example, `/media/**/*.png` matches all PNGs in `media` and it's subdirectories.

```rust,editable
{{#include ../../../crates/cats/filesystem/examples/directory_traversal/png.rs:example}}
```

## Find All Files with Given Pattern, Ignoring Filename Case {#find-all-files-with-given-pattern-ignoring-filename-case}

[![glob][c~glob~docs~badge]][c~glob~docs] [![glob~crates.io][c~glob~crates.io~badge]][c~glob~crates.io] [![glob~github][c~glob~github~badge]][c~glob~github] [![glob~lib.rs][c~glob~lib.rs~badge]][c~glob~lib.rs]{{hi:glob}} [![cat~filesystem][cat~filesystem~badge]][cat~filesystem]{{hi:Filesystem}}

Find all image files in the `/media/` directory matching the `img_[0-9][0-9]*.png` pattern.

A custom [`glob::MatchOptions`][c~glob::MatchOptions~docs]↗{{hi:glob::MatchOptions}} struct is passed to the [`glob::glob_with`][c~glob::glob_with~docs]↗{{hi:glob::glob_with}} function making the glob{{hi:glob}} pattern case insensitive while keeping the other options [`std::default::Default`][c~std::default::Default~docs]↗{{hi:std::default::Default}}.

```rust,editable
{{#include ../../../crates/cats/filesystem/examples/directory_traversal/ignore_case.rs:example}}
```

## `globset` {#globset}

[![globset][c~globset~docs~badge]][c~globset~docs] [![globset~crates.io][c~globset~crates.io~badge]][c~globset~crates.io] [![globset~github][c~globset~github~badge]][c~globset~github] [![globset~lib.rs][c~globset~lib.rs~badge]][c~globset~lib.rs]{{hi:globset}}{{hi:Glob}}{{hi:Multiple}}{{hi:Pattern}}{{hi:Regex}}{{hi:Set}}

[`globset`][c~globset~docs]↗{{hi:globset}} allows multiple globs to be evaluated at once. Glob set matching is the process of matching one or more [`glob`][c~glob~docs]↗{{hi:glob}} patterns against a single candidate path simultaneously, and returning all of the globs that matched.

```rust,editable
{{#include ../../../crates/cats/filesystem/examples/directory_traversal/globset.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write; review](https://github.com/john-cd/rust_howto/issues/358)
</div>
