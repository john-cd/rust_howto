// TODO Reviewed ??-??-2026.

#![expect(
    dead_code,
    reason = "the snippet declares items to illustrate them, not to call them"
)]
// ANCHOR: example
//! Making the futures of a trait method `Send` so the trait can be used with
//! `tokio::spawn`: either declare `-> impl Future<Output = T> + Send`, or let
//! `trait_variant` generate a `Send` version of the trait.

use std::collections::HashMap;
use std::future::Future;

/// Style 1: return-position `impl Trait` in a trait (RPITIT) with an explicit
/// `Send` bound.
trait Refresh {
    /// Refreshes the state and returns a new version number.
    fn refresh(&mut self) -> impl Future<Output = Result<u32, String>> + Send;
}

/// Implementors may still write `async fn`: the compiler checks the bound.
struct Counter(u32);

impl Refresh for Counter {
    /// Increments and returns the counter value.
    async fn refresh(&mut self) -> Result<u32, String> {
        self.0 += 1;
        Ok(self.0)
    }
}

/// Runs a generic `Refresh` on another task.
async fn refresh_in_task<R: Refresh + Send + 'static>(
    mut r: R,
) -> Result<u32, String> {
    tokio::spawn(async move { r.refresh().await })
        .await
        .map_err(|e| e.to_string())?
}

/// Style 2: `trait_variant` generates `SendStore`, whose futures are `Send`,
/// from the plain trait `LocalStore`.
#[trait_variant::make(SendStore: Send)]
trait LocalStore {
    /// Looks up a key.
    async fn get(&self, key: &str) -> Option<String>;
    /// Stores a value.
    async fn put(&mut self, key: &str, value: String);
}

/// An in-memory store; it implements both traits (`LocalStore` for free
/// functions on one thread, `SendStore` for spawned tasks).
#[derive(Default)]
struct MemoryStore(HashMap<String, String>);

impl SendStore for MemoryStore {
    /// Returns the value associated with `key`, if any.
    async fn get(&self, key: &str) -> Option<String> {
        self.0.get(key).cloned()
    }

    /// Stores a cloned key and owned value.
    async fn put(&mut self, key: &str, value: String) {
        self.0.insert(key.to_owned(), value);
    }
}

/// Stores a value and reads it back on a spawned task.
async fn roundtrip_in_task<S: SendStore + Send + 'static>(
    mut store: S,
) -> Option<String> {
    tokio::spawn(async move {
        store.put("k", "v".to_owned()).await;
        store.get("k").await
    })
    .await
    .ok()
    .flatten()
}

/// Spawns both kinds of trait on the multi-threaded runtime.
#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() {
    assert_eq!(refresh_in_task(Counter(0)).await, Ok(1));
    assert_eq!(
        roundtrip_in_task(MemoryStore::default()).await.as_deref(),
        Some("v")
    );
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Checks both `Send` trait styles on a multi-threaded runtime.
    /// Runs the rpitit send example through its entry point.
    #[test]
    fn test_rpitit_send_example_entry_point() {
        main();
    }
}
