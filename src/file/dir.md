# Directory Traversal

## File names that have been modified in the last 24 hours

[![std-badge]][std] [![cat-filesystem-badge]][cat-filesystem]

Gets the current working directory by calling [`env::current_dir`],
then for each entries in [`fs::read_dir`], extracts the
[`DirEntry::path`] and gets the metadata via [`fs::Metadata`]. The
[`Metadata::modified`] returns the [`SystemTime::elapsed`] time since
last modification. [`Duration::as_secs`] converts the time to seconds and
compared with 24 hours (24 *60* 60 seconds). [`Metadata::is_file`] filters
out directories.

```rust,editable
{#include ../../../deps/examples/modified.rs}
```

## Find loops for a given path

[![same_file-badge]][same_file] [![cat-filesystem-badge]][cat-filesystem]

Use [`same_file::is_same_file`] to detect loops for a given path.
For example, a loop could be created on a Unix system via symlinks:

```bash
mkdir -p /tmp/foo/bar/baz
ln -s /tmp/foo/  /tmp/foo/bar/baz/qux
```

The following would assert that a loop exists.

```rust,editable,no_run
{#include ../../../deps/examples/loops.rs}
```

## Recursively find duplicate file names

[![walkdir-badge]][walkdir] [![cat-filesystem-badge]][cat-filesystem]

Find recursively in the current directory duplicate filenames,
printing them only once.

```rust,editable,no_run
{#include ../../../deps/examples/duplicate-name.rs}
```

## Recursively find all files with given predicate

[![walkdir-badge]][walkdir] [![cat-filesystem-badge]][cat-filesystem]

Find JSON files modified within the last day in the current directory.
Using [`follow_links`] ensures symbolic links are followed like they were
normal directories and files.

```rust,editable,no_run
{#include ../../../deps/examples/find-file.rs}
```

## Traverse directories while skipping dotfiles

[![walkdir-badge]][walkdir] [![cat-filesystem-badge]][cat-filesystem]

Uses [`filter_entry`] to descend recursively into entries passing the
`is_not_hidden` predicate thus skipping hidden files and directories.
 [`Iterator::filter`] applies to each [`WalkDir::DirEntry`] even if the parent
 is a hidden directory.

Root dir `"."` yields through [`WalkDir::depth`] usage in `is_not_hidden`
predicate.

```rust,editable,no_run
{#include ../../../deps/examples/skip-dot.rs}
```

## Recursively calculate file sizes at given depth

[![walkdir-badge]][walkdir] [![cat-filesystem-badge]][cat-filesystem]

Recursion depth can be flexibly set by [`WalkDir::min_depth`] & [`WalkDir::max_depth`] methods.
Calculates sum of all file sizes to 3 subfolders depth, ignoring files in the root folder.

```rust,editable
{#include ../../../deps/examples/sizes.rs}
```

## Find all png files recursively

[![glob-badge]][glob] [![cat-filesystem-badge]][cat-filesystem]

Recursively find all PNG files in the current directory.
In this case, the `**` pattern matches the current directory and all subdirectories.

Use the `**` pattern in any path portion. For example, `/media/**/*.png`
matches all PNGs in `media` and it's subdirectories.

```rust,editable,no_run
{#include ../../../deps/examples/png.rs}
```

## Find all files with given pattern ignoring filename case

[![glob-badge]][glob] [![cat-filesystem-badge]][cat-filesystem]

Find all image files in the `/media/` directory matching the `img_[0-9]*.png` pattern.

A custom [`MatchOptions`] struct is passed to the [`glob_with`] function making the glob pattern case insensitive while keeping the other options [`Default`].

```rust,editable,no_run
{#include ../../../deps/examples/ignore-case.rs}
```

[`follow_links`]: https://docs.rs/walkdir/*/walkdir/struct.WalkDir.html#method.follow_links
[`same_file::is_same_file`]: https://docs.rs/same-file/*/same_file/fn.is_same_file.html
[`WalkDir::max_depth`]: https://docs.rs/walkdir/*/walkdir/struct.WalkDir.html#method.max_depth
[`WalkDir::min_depth`]: https://docs.rs/walkdir/*/walkdir/struct.WalkDir.html#method.min_depth
[`DirEntry::path`]: https://doc.rust-lang.org/std/fs/struct.DirEntry.html#method.path
[`Duration::as_secs`]: https://doc.rust-lang.org/std/time/struct.Duration.html#method.as_secs
[`env::current_dir`]: https://doc.rust-lang.org/std/env/fn.current_dir.html
[`fs::Metadata`]: https://doc.rust-lang.org/std/fs/struct.Metadata.html
[`fs::read_dir`]: https://doc.rust-lang.org/std/fs/fn.read_dir.html
[`Metadata::is_file`]: https://doc.rust-lang.org/std/fs/struct.Metadata.html#method.is_file
[`Metadata::modified`]: https://doc.rust-lang.org/std/fs/struct.Metadata.html#method.modified
[`SystemTime::elapsed`]: https://doc.rust-lang.org/std/time/struct.SystemTime.html#method.elapsed
[`filter_entry`]: https://docs.rs/walkdir/*/walkdir/struct.IntoIter.html#method.filter_entry
[`Iterator::filter`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter
[`WalkDir::depth`]: https://docs.rs/walkdir/*/walkdir/struct.DirEntry.html#method.depth
[`WalkDir::DirEntry`]: https://docs.rs/walkdir/*/walkdir/struct.DirEntry.html
[`Default`]: https://doc.rust-lang.org/std/default/trait.Default.html
[`glob_with`]: https://docs.rs/glob/*/glob/fn.glob_with.html
[`MatchOptions`]: https://docs.rs/glob/*/glob/struct.MatchOptions.html
{{#include ../refs/link-refs.md}}
