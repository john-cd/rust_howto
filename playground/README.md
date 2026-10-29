<!-- TODO Reviewed ??-??-2026 -->

# Playground

This folder contains code for testing and exploration. The remaining `examples_wip`
examples are scratch material, with these review dispositions:

- `anyhow_uptime.rs` is Linux-only because it reads `/proc/uptime`; it needs a portable
  source and exit-status behavior before publication.
- `asref2.rs` frames `AsRef` as a last-resort reference conversion rather than
  inheritance and explains when a narrow accessor or domain trait is clearer.
- `split.rs` implements a narrow slash parser whose behavior does not match its claim
  to return a directory, file name, and extension. Use `std::path` before promoting it.

The reviewed module paths example was moved to the published Modules and Paths chapter.
`bits_and_pieces` remains an archival scratch area: its old `tool_lib` and parser
prototypes have no package or book entry points, and no chapter references them. None of
the remaining fragments is ready to publish. The commented-out ASCII scan in
`just/justfile.2` had no caller and was removed during this review.
