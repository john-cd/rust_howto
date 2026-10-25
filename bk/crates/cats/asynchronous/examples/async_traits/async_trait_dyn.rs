#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! `#[async_trait]` for trait objects.
//!
//! Native `async fn` in traits is not dyn-compatible: `Box<dyn Store>` does
//! not compile. `async_trait` rewrites each method to return a
//! `Pin<Box<dyn Future + Send>>`, at the cost of one heap allocation per call.
//! The `dynosaur` crate is a newer alternative that keeps native syntax.

use async_trait::async_trait;

/// A storage backend, usable as `dyn Store`.
#[async_trait]
trait Store: Send + Sync {
    /// The name of the backend.
    fn name(&self) -> &'static str;
    /// Reads a value.
    async fn read(&self, key: &str) -> Option<String>;
}

/// Always answers with the same value.
struct Constant(&'static str);

#[async_trait]
impl Store for Constant {
    /// Names this backend for display in the results.
    fn name(&self) -> &'static str {
        "constant"
    }

    /// Returns the constant value for every key.
    async fn read(&self, _key: &str) -> Option<String> {
        Some(self.0.to_owned())
    }
}

/// Never has anything.
struct Empty;

#[async_trait]
impl Store for Empty {
    /// Names this backend for display in the results.
    fn name(&self) -> &'static str {
        "empty"
    }

    /// Reports that the empty backend has no stored values.
    async fn read(&self, _key: &str) -> Option<String> {
        None
    }
}

/// Queries every backend in a heterogeneous list.
async fn read_all(
    stores: &[Box<dyn Store>],
    key: &str,
) -> Vec<(&'static str, Option<String>)> {
    let mut out = Vec::new();
    for store in stores {
        out.push((store.name(), store.read(key).await));
    }
    out
}

/// Builds a `Vec<Box<dyn Store>>` and queries it.
#[tokio::main]
async fn main() {
    let stores: Vec<Box<dyn Store>> =
        vec![Box::new(Constant("hello")), Box::new(Empty)];
    let results = read_all(&stores, "k").await;
    assert_eq!(
        results,
        vec![("constant", Some("hello".to_owned())), ("empty", None)]
    );
}
// ANCHOR_END: example

/// Runs the example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Checks heterogeneous async stores through trait objects.
    #[test]
    fn test_main() {
        main();
    }
}
