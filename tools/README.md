# Tools

This workspace contains tools written in Rust, which you may use to manage the book.

- `clean` is a very simple tool to clean `crates/**/temp` before and after testing.
- `crate_indices` creates the `crates by category` and `crates (alphabetical)` pages.
- `templ` generate markdown links and reference definitions. The outputs can be copy-pasted into the book's markdown where needed.
- `tool_lib` is a Rust library shared between all Rust CLI tools.
