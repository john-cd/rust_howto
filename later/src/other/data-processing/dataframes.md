<!-- TODO Reviewed ??-??-2026 -->

# Dataframes

{{#include dataframes.incl.md}}

## Manipulate Data in a Tabular Format {#dataframes}

### Filter and Aggregate with Polars {#polars}

[![polars][c~polars~docs~badge]][c~polars~docs]{{hi:polars}} [![`polars` Book][c~polars~book~badge]][c~polars~book] [![`polars` GitHub][c~polars~repo~badge]][c~polars~repo]{{hi:Dataframes}}

Polars is a Rust DataFrame library built on the {{i:Apache Arrow}} columnar memory model. The example builds a DataFrame with inline data, filters it lazily, groups by region, and collects sorted revenue totals. Its assertions run with the `other` crate tests.

[docs.pola.rs][c~polars~docs]↗.

```rust,editable
{{#include ../../../crates/other/examples/polars/polars.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[data_processing | Data and ETL]].
- [[python | Python]].
- [[mathematics | Mathematics]].
- [[visualization | Visualization]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  [dataframes: organize / edit](https://github.com/john-cd/rust_howto/issues/587)
</div>
