<!-- TODO Reviewed ??-??-2026. -->

# Visualization

Run from `bk/crates`:

```console
cargo run --quiet --locked -p visualization --example visualization -- plotters > chart.svg
cargo run --locked -p visualization --example visualization -- plotly
cargo run --locked -p visualization --example terminal_table
cargo test --locked -p visualization --examples
```

Open `chart.svg` to inspect the static chart. Open `temp/plot.html` in a browser with access to Plotly's CDN for the interactive example. The terminal table prints exact reported values and units.

## Chartistry browser example

Install [Trunk](https://github.com/trunk-rs/trunk/releases). From `bk/crates/cats/visualization`:

```console
rustup target add wasm32-unknown-unknown
trunk serve examples/chartistry/index.html --locked
```

Open the URL printed by Trunk and use **Add sample** to update the chart. The HTML entry point enables the optional `chartistry` feature.

The recipes are in [Visualization](../../../src/categories/visualization/index.md).
