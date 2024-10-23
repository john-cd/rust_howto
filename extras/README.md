# Tools

This folder contains shell scripts and tools written in Rust, which you may use to manage the book.

- `hide` contains scripts to hide or unhide markdown chapters.
- `tools` contains the Rust library to be shared between all Rust CLI tools.
- `tools/src/bin` contains (for now) one CLI tool: `templ`, a tool that generate markdown links and reference definitions. The outputs can be copy-pasted into the book's markdown where needed.
