<!-- TODO Reviewed ??-??-2026 -->

# File Listing and Display

{{#include filesystem_cli.incl.md}}

## List Files {#list-files}

Use [`eza`][eza~repo]↗, a maintained replacement for the discontinued `exa`, or [`lsd`][c~lsd~repo]↗ for a colored, detailed directory listing. Both provide options for long listings and tree views.

### `eza` {#exa}

```bash
cargo install eza --locked
eza --long --all --git
```

The `#exa` anchor is retained for existing links; new references should use `eza`.

### `lsd` {#lsd}

```bash
cargo install lsd --locked
lsd --long --almost-all
```

## Find Files {#find-files}

[`fd`][c~fd-find~repo]↗ is a fast, user-friendly file finder. It respects ignore files by default.

```bash
cargo install fd-find --locked
fd -e rs . src
```

## Search File Contents {#search-file-contents}

[`ripgrep`][c~ripgrep~repo]↗ provides the `rg` command for recursive, ignore-aware text search.

```bash
cargo install ripgrep --locked
rg -n 'TODO|FIXME' src
```

## View File Contents {#view-file-contents}

[`bat`][c~bat~repo]↗ is a `cat` replacement with syntax highlighting, line numbers, and Git change markers.

### `bat` {#bat}

```bash
cargo install bat --locked
bat README.md
```

## Analyze Disk Usage {#analyze-disk-usage}

[`dust`][dust~repo]↗ displays disk usage as a sorted directory tree. [`dua`][dua~repo]↗ offers an interactive disk-usage explorer.

```bash
cargo install du-dust --locked
dust -d 2 .

cargo install dua-cli --locked
dua i .
```

## Rename and Bulk Edit {#rename-and-bulk-edit}

[`sd`][c~sd~repo]↗ uses regular expressions for focused text replacements. Review the diff after it edits files in place.

```bash
cargo install sd --locked
sd 'OldName' 'NewName' src/main.rs
git diff -- src/main.rs
```

`sd` changes file contents, not filenames. To rename a tracked file, use `git mv` and inspect the result:

```bash
git mv -- src/old_name.rs src/new_name.rs
git status --short
```

For files outside a Git repository, use the shell's `mv` command. Check that the destination is available before renaming.

## Related Topics {#related-topics .skip}

- [[command-line-interface | Command Line Interface]].
- [[external_commands | External Commands]].
- [[filesystem | Filesystem]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
