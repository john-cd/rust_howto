<!-- TODO Reviewed ??-??-2026. -->

# Present Measurements in a Terminal Table

Use a table when the reader needs individual values, such as comparing the
distance measured at two stations. [tabled](https://docs.rs/tabled/) provides
a builder for a number of rows known only at runtime. This example labels the
units in the header and renders plain ASCII without terminal escape codes.

```rust,editable
{{#include ../../../crates/cats/visualization/examples/terminal_table.rs:example}}
```

Run from the repository root:

```console
just cargo run --manifest-path later/crates/Cargo.toml --locked -p visualization --example terminal_table
```

The table reports North at 12.5 m and South at 8.0 m. Keep the original numeric
values for calculations and use the formatted table for human output. For
machine consumption, use [CSV](../../other/data-processing/csv.md). When the
shape of a trend matters more than individual values, use
[Plotters](visualization.md#plotters).
