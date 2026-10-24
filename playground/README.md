<!-- TODO Reviewed ??-??-2026 -->

# Playground

This folder contains code for testing and exploration. The remaining `examples_wip`
examples are scratch material, with these review dispositions:

- `anyhow_uptime.rs` is Linux-only because it reads `/proc/uptime`; it needs a portable
  source and exit-status behavior before publication.
- `asref2.rs` presents `AsRef` as an inheritance substitute, which conflates a cheap
  reference conversion with a subtype relationship. It needs a new design before
  publication.
- `split.rs` implements a narrow slash parser whose behavior does not match its claim
  to return a directory, file name, and extension. Use `std::path` before promoting it.

The reviewed module paths example was moved to the published Modules and Paths chapter.
Other files under `bits_and_pieces` remain experiments without a chapter-ready scope.
