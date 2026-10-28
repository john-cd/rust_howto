// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Turns an address into coordinates with the `geocoding` crate.
//!
//! The provider sits behind the `Forward` trait, so a cache and a fake
//! implementation for the tests can wrap it.

use std::collections::HashMap;

use geocoding::Forward;
use geocoding::GeocodingError;
use geocoding::Openstreetmap;
use geocoding::Point;

/// Wraps any forward geocoder and remembers its answers.
struct CachedGeocoder<G> {
    /// The provider to ask on a cache miss.
    inner: G,
    /// The first match for each normalized query. `None` records "not found".
    cache: HashMap<String, Option<Point<f64>>>,
}

impl<G: Forward<f64>> CachedGeocoder<G> {
    /// Creates an empty cache around `inner`.
    fn new(inner: G) -> Self {
        Self {
            inner,
            cache: HashMap::new(),
        }
    }

    /// Returns the coordinates (longitude, latitude) of the best match.
    fn locate(
        &mut self,
        query: &str,
    ) -> Result<Option<Point<f64>>, GeocodingError> {
        let key = normalize(query);
        if let Some(hit) = self.cache.get(&key) {
            return Ok(*hit);
        }
        let first = self.inner.forward(&key)?.into_iter().next();
        self.cache.insert(key, first);
        Ok(first)
    }
}

/// Normalizes free text so that equivalent queries share a cache entry.
fn normalize(query: &str) -> String {
    query
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
}

/// Queries OpenStreetMap Nominatim.
///
/// This function is not called by the tests: it needs the network and is
/// subject to the Nominatim usage policy (at most one request per second).
fn locate_online(address: &str) -> Result<Option<Point<f64>>, GeocodingError> {
    let mut geocoder = CachedGeocoder::new(Openstreetmap::new());
    geocoder.locate(address)
}

/// A geocoder that knows one place and counts the questions it is asked.
struct Gazetteer {
    /// The number of calls to `forward`.
    calls: std::cell::Cell<usize>,
}

impl Forward<f64> for Gazetteer {
    fn forward(
        &self,
        address: &str,
    ) -> Result<Vec<Point<f64>>, GeocodingError> {
        self.calls.set(self.calls.get() + 1);
        if address == "eiffel tower, paris" {
            Ok(vec![Point::new(2.2945, 48.8584)])
        } else {
            Ok(vec![])
        }
    }
}

/// Geocodes an address twice and asks the provider only once.
fn main() -> Result<(), GeocodingError> {
    let gazetteer = Gazetteer {
        calls: std::cell::Cell::new(0),
    };
    let mut geocoder = CachedGeocoder::new(gazetteer);

    let first = geocoder.locate("Eiffel Tower,  Paris")?;
    let second = geocoder.locate("eiffel tower, paris")?;
    assert_eq!(first, Some(Point::new(2.2945, 48.8584)));
    assert_eq!(first, second);
    assert_eq!(geocoder.inner.calls.get(), 1);

    // A failed search is cached too.
    assert_eq!(geocoder.locate("Atlantis")?, None);
    assert_eq!(geocoder.locate("atlantis")?, None);
    assert_eq!(geocoder.inner.calls.get(), 2);
    Ok(())
}
// ANCHOR_END: example

pub fn run() -> Result<(), GeocodingError> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the geocode example through its entry point.
    #[test]
    fn test_geocode_example_entry_point() -> Result<(), GeocodingError> {
        main()
    }

    #[test]
    fn normalize_collapses_spaces_and_case() {
        assert_eq!(normalize("  Paris ,   FRANCE "), "paris , france");
    }
}
