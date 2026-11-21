<!-- TODO Reviewed ??-??-2026. -->

# `example-integrity`

This read-only tool inventories the book's Markdown chapters, recursive includes,
Rust code snippets, Cargo targets, reachable Rust modules, and documented exceptions.
It inventories `bk/src` and `later/src` and recognizes visible command walkthroughs
independently of Cargo-backed examples.
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

The same binary replaces the example test-convention survey and the legacy
unused-example listing:

```sh
cargo run --locked -p example-integrity -- tests [--summary] [--csv] [ROOT ...]
cargo run --locked -p example-integrity -- unused-examples [--book-root <PATH>] [--summary]
```

`tests` is informational unless `--fail` is supplied. Its default scan roots are
`bk/crates`, `later/crates`, `xmpl`, and `tools`. The unused-example view scans
the selected book's `crates/` tree and recognizes native includes from `bk/src`
and `later/src`. The existing no-subcommand invocation and JSON schema remain
available for the strict chapter/include audit.
