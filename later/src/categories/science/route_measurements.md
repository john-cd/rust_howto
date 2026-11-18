<!-- TODO Reviewed ??-??-2026. -->

# Compare Route Measurements with a Model

A route logger recorded three positions at known times. Use [geo](https://docs.rs/geo/)
to sum the straight segments between consecutive readings, then compare the
measured duration with a model that assumes a constant speed of 2 m/s.

```rust,editable
{{#include ../../../crates/cats/science_geo/examples/route_measurements.rs:example}}
```

From the repository root, run:

```console
just cargo run --manifest-path later/crates/Cargo.toml --locked -p science_geo --example route_measurements
```

The fixture travels 17.0 m in 11.0 s. At 2 m/s the same route would take 8.5 s,
so the model underestimates the measured duration by 2.5 s. The test also
rejects repeated timestamps.

The coordinates are planar meters and the timestamps are seconds from a
common origin. Straight segments approximate travel between observations and
can miss curves. For longitude and latitude, follow the
[geographic distance examples](../science_geo/geo.md). This example uses
the Rust geometry algorithms and needs no GEOS installation. For operations
that use the optional native library, see the Geo chapter's prerequisites.

Use the [simulation chapter](../simulation/simulation.md) when the model needs
time steps or state. For a sampled signal whose frequency is the quantity of
interest, follow the [sample analysis recipe](../science_neuroscience/neuroscience.md#fft-signal).
