<!-- TODO Reviewed ??-??-2026. -->

# Science Geo

This crate contains code examples for the **Science Geo** category on crates.io. It demonstrates integration with key ecosystem crates including `geo`, `geocoding`, `haversine`, `us-state-info`.

## Examples Covered

- **Geo** (`examples/geo/`): Demonstrates concepts like `geo`, `geocode`, `haversine`, `us_states`.
- **Route measurements** (`examples/route_measurements.rs`): Compares measured
  travel time with a constant speed model using planar distances from `geo`.

## Usage

To check or run examples in this crate:

```console
cargo check --package science_geo --examples
cargo test --locked -p science_geo --examples
cargo run --locked -p science_geo --example route_measurements
```

The walkthroughs are in `later/src/categories/science_geo` and
`later/src/categories/science`.
