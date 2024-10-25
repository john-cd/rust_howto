# Tools

This folder contains tools written in Rust, which you may use to manage the book.

- `src/lib.rs` contains the Rust library to be shared between all Rust CLI tools.
- `src/bin` contains (for now) the following CLI tools:
  - `templ`, which generate markdown links and reference definitions.
  - `crate_indices`, which creates the `crates by category` page.
  - The outputs can be copy-pasted into the book's markdown where needed.
