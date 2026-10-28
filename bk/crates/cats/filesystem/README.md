<!-- TODO Reviewed ??-??-2026 -->

# Filesystem

This crate contains code examples for the **Filesystem** category on crates.io. It demonstrates integration with key ecosystem crates including `anyhow`, `atomicwrites`, `camino`, `clap`, `directories`, `dirs`.

## Examples Covered

- **Atomic Write** (`examples/atomic_write/`): Demonstrates concepts like `atomic_write`.
- **Directories** (`examples/directories/`): Demonstrates concepts like `cwd`, `manipulate_dirs`, `remove_dir_all`.
- **Directory Traversal** (`examples/directory_traversal/`): Demonstrates concepts like `duplicate_name`, `find_file`, `globset`, `ignore`, `ignore_case`, `loops`, `modified`, `png`, `sizes`, `skip_dot`.
- **File Watching** (`examples/file_watching/`): Demonstrates concepts like `notify`.
- **Metadata** (`examples/metadata/`): Demonstrates concepts like `infer`, `metadata`.
- **Paths** (`examples/paths/`): Demonstrates concepts like `camino`, `canonicalize`, `dunce`, `manipulate_paths`.
- **Read Write** (`examples/read_write/`): Demonstrates concepts like `memmap2`, `read_file`, `same_file`.
- **Stdin Stdout** (`examples/stdin_stdout/`): Demonstrates concepts like `stdin_stdout`.
- **Symbolic Links** (`examples/symbolic_links/`): Demonstrates concepts like `symbolic_links`.
- **Tempfile** (`examples/tempfile/`): Demonstrates concepts like `tempfile`.
- **User Directories** (`examples/user_directories/`): Demonstrates concepts like `directories`, `dirs`.
- **Standalone Examples**: `open`.

## Usage

To check or run examples in this crate:

```console
cargo check --package filesystem --examples
cargo test --package filesystem
```

Code examples correspond to chapters in `bk/src/categories/filesystem` or related book sections.
