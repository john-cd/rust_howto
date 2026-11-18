<!-- TODO Reviewed ??-??-2026. -->

# Windows APIs

This crate contains code examples for the **Windows APIs** category on crates.io. It demonstrates integration with key ecosystem crates including `windows` and `winapi`.

## Examples Covered

- **Windows** (`examples/windows/`): Demonstrates concepts like `winapi`, `windows`.

## Usage

To check or run examples in this crate:

```console
cargo check --package os_windows_apis --examples
cargo test --package os_windows_apis --example windows
```

Code examples correspond to chapters in `bk/src/categories/os_windows-apis` or related book sections.

The message-box tests are ignored by default because they require an interactive Windows desktop. Run `cargo run --package os_windows_apis --example windows -- windows` (or `-- winapi`) to display one dialog. Dismiss it to finish the command.
