<!-- TODO Reviewed ??-??-2026 -->

# Utilities Library

This crate provides shared helper functions, filesystem utilities, test fixtures, and environment setup utilities used across the code examples and integration tests in the book repository.

## Capabilities

- **Filesystem Utilities**: Helper routines for creating temporary directories, searching files, and managing test artifacts safely.
- **Test Helpers**: Shared assertions and test setup wrappers.

## Usage

Import `utils` as a dependency in workspace crates requiring shared helper routines:

```rust
use utils::*;
```

To test the utility library:

```console
cargo test --package utils
```
