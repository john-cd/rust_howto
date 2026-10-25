<!-- TODO Reviewed ??-??-2026 -->

# Snapshot Testing

{{#include snapshot_testing.incl.md}}

A snapshot test compares a value with a stored reference (the snapshot) instead of with a hand-written expectation. It suits output that is large or tedious to write by hand, such as rendered text, error messages, generated code or serialized data. The risk is accepting a changed snapshot without reading it, so review every diff, and prefer small, focused snapshots.

See [[testing | Testing]] for the basics of [`insta`][c~insta~docs]↗{{hi:insta}}: inline snapshots, `cargo insta review` and file snapshots.

## Snapshot Structured Data with `insta` Redactions {#insta-redactions}

[![insta][c~insta~docs~badge]][c~insta~docs]{{hi:insta}} [![insta~crates.io][c~insta~crates.io~badge]][c~insta~crates.io] [![insta~repo][c~insta~repo~badge]][c~insta~repo] [![cat~development-tools::testing][cat~development-tools::testing~badge]][cat~development-tools::testing]{{hi:Snapshot testing}}

`assert_json_snapshot!` (feature `json`) serializes a value with `serde` and compares the pretty-printed JSON; `assert_debug_snapshot!` uses the `Debug` output instead. Values that change between runs, such as ids, timestamps and random tokens, would make the snapshot fail every time. With the `redactions` feature, a selector such as `".id" => "[id]"` replaces the value before comparison. Selectors can also match array items (`".items[]"`) and apply sorting, rounding or a dynamic check.

```rust,editable
{{#include ../../../crates/cats/development_tools_testing/examples/test_techniques/snapshot_redactions.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[testing | Testing]] describes unit, integration, and documentation tests.
- [[property_based_testing | Property-based Testing]] explores many generated inputs rather than one stored output.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
