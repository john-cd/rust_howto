<!-- TODO Reviewed ??-??-2026 -->

# NOTES

A few tests use attributes that must be applied at the crate level.
As such, they must be placed at the root of the `tests` folder.

## Test Organization
- Inline unit tests are placed in `#[cfg(test)] mod tests` at the bottom of example modules.
- Integration tests requiring process or crate-level isolation are located in the `tests/` directory.
