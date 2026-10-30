# `example-integrity`

This read-only tool inventories the book's Markdown chapters, recursive includes,
Rust code snippets, Cargo targets, reachable Rust modules, and documented exceptions.
It emits stable JSON to stdout. Pass `--fail` to return a failing status when the
report contains integrity errors.

Run it from `tools/` with:

```sh
just audit-examples
```

The report uses repository-relative slash-separated paths so identical basenames in
different crates remain distinct. It follows native mdBook includes, checks Rust
anchors and fencing, maps included files to tested Cargo targets, retains `cfg`
conditions on module paths, and identifies chapters absent from `SUMMARY.md`.
