<!-- TODO Reviewed ??-??-2026. -->

# Visualization

Run from `later/crates`:

```console
cargo run --quiet --locked -p visualization --example visualization -- plotters > chart.svg
cargo run --locked -p visualization --example visualization -- plotly
cargo run --locked -p visualization --example terminal_table
cargo test --locked -p visualization --examples
```

Open `chart.svg` to inspect the static chart. Open `temp/plot.html` in a browser with access to Plotly's CDN for the interactive example. The terminal table prints exact reported values and units.

The Chartistry subcommand builds a Leptos component for reference. The upstream examples show how to mount it in a browser. The recipes are in [Visualization](../../../src/categories/visualization/index.md).
