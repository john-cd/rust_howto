<!-- TODO Reviewed ??-??-2026 -->

# Geo-spatial Science

{{#include geo.incl.md}}

## Process Spatial Information, Maps, Navigation Data, and Geographic Information Systems {#geo}

[![geo][c~geo~docs~badge]][c~geo~docs] [![geo~crates.io][c~geo~crates.io~badge]][c~geo~crates.io] [![geo~repo][c~geo~repo~badge]][c~geo~repo] [![geo~lib.rs][c~geo~lib.rs~badge]][c~geo~lib.rs]{{hi:geo}}{{hi:GIS}}{{hi:Geography}}{{hi:geo}}{{hi:Geospatial}} [![cat~science::geo][cat~science::geo~badge]][cat~science::geo]{{hi:Geospatial}}

[`geo`][c~geo~docs]↗{{hi:geo}} provides geospatial primitives and algorithms.

```rust,editable
{{#include ../../../crates/cats/science_geo/examples/geo/geo.rs:example}}
```

## Compute the Distance Between Two Places {#haversine}

[![haversine][c~haversine~docs~badge]][c~haversine~docs]{{hi:haversine}} [![geo][c~geo~docs~badge]][c~geo~docs]{{hi:geo}} [![cat~science::geo][cat~science::geo~badge]][cat~science::geo]{{hi:Distance}}{{hi:Great-circle distance}}

The haversine formula gives the great-circle distance between two points on a sphere. [`haversine::distance`][c~haversine::distance~docs]↗{{hi:haversine::distance}} is a one-function crate; [`geo::Haversine`][c~geo::Haversine~docs]↗{{hi:geo::Haversine}} offers the same calculation through the `geo::Distance` trait, with the other `geo` geometry types.

Mind the order of the coordinates: `haversine::Location` names its fields `latitude` and `longitude`, while a `geo::Point` is `(x, y)`, that is `(longitude, latitude)`. `haversine` returns kilometers or miles, `geo` returns meters.

```rust,editable
{{#include ../../../crates/cats/science_geo/examples/geo/haversine.rs:example}}
```

The haversine formula treats the Earth as a sphere, which is good to roughly 0.5 %. When you need the distance to the meter, use an ellipsoidal model such as [`geographiclib-rs`][c~geographiclib-rs~docs]↗{{hi:geographiclib-rs}}.

## Geocode an Address {#geocode}

[![geocoding][c~geocoding~docs~badge]][c~geocoding~docs]{{hi:geocoding}} [![cat~science::geo][cat~science::geo~badge]][cat~science::geo]{{hi:Geocoding}}

[`geocoding`][c~geocoding~docs]↗{{hi:geocoding}} turns an address into coordinates through providers such as OpenStreetMap Nominatim ([`geocoding::Openstreetmap`][c~geocoding::Openstreetmap~docs]↗{{hi:geocoding::Openstreetmap}}) and OpenCage. Each provider implements the [`geocoding::Forward`][c~geocoding::Forward~docs]↗{{hi:geocoding::Forward}} trait. Code against the trait, and you can add a cache and substitute a fake provider in tests. This example does both, so the unit tests need no network.

```rust,editable,no_run
{{#include ../../../crates/cats/science_geo/examples/geo/geocode.rs:example}}
```

Before geocoding at scale:

- Read the terms of the provider. The public Nominatim service allows at most one request per second and requires an identifying `User-Agent`. Commercial providers need an API key and meter requests. See [Handle a Rate-limited API][p~rate-limited-api] to limit requests on the client side.
- Cache the results, since the same address is often asked for repeatedly. The example keeps a plain `HashMap`; use a cache with expiry such as [`moka`][c~moka~docs]↗{{hi:moka}} in a long-running program, and check the provider's rules on storing results.
- Normalize the query before you look it up, so that equivalent spellings share a cache entry.
- Remember the order of the result: `geocoding` returns `Point::new(longitude, latitude)`.

## Recognize US States {#us-states}

[![us-state-info][c~us-state-info~docs~badge]][c~us-state-info~docs]{{hi:us-state-info}} [![cat~science::geo][cat~science::geo~badge]][cat~science::geo]{{hi:US states}}

[`us_state_info::State`][c~us-state-info::State~docs]↗{{hi:us_state_info::State}} is an enumeration of the US states. Converting a `String` with `State::try_from` accepts both the full name and the two-letter abbreviation, ignoring case and spaces. This helps to normalize free-text locations before geocoding or grouping.

```rust,editable
{{#include ../../../crates/cats/science_geo/examples/geo/us_states.rs:example}}
```

## Related Topics {#related-topics .skip}

- GIS data formats.
- Mapping and visualization.
- Spatial analysis.
- Scientific and geospatial workflows.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[geo: write](https://github.com/john-cd/rust_howto/issues/474)
[geocode addresses and compute distances](https://github.com/john-cd/rust_howto/issues/1942)
</div>
