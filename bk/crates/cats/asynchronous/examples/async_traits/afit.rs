#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! A native `async fn` in a trait (AFIT), called through generics.
//!
//! Decision table:
//!
//! | Need                                   | Use                                      |
//! |----------------------------------------|------------------------------------------|
//! | Static dispatch, single-threaded       | native `async fn` in the trait           |
//! | Static dispatch, `tokio::spawn`        | `-> impl Future<Output = T> + Send` or `trait_variant` |
//! | `dyn Trait` (a `Vec<Box<dyn Store>>`)  | `#[async_trait]` (boxes each future)     |
//! | Local executors (`!Send` futures)      | native `async fn`; no `Send` bound       |

use std::collections::HashMap;

/// A key-value store with an asynchronous API.
trait Store {
    /// Looks up a key.
    async fn get(&self, key: &str) -> Option<String>;
}

/// An in-memory implementation.
struct MemoryStore(HashMap<String, String>);

impl Store for MemoryStore {
    async fn get(&self, key: &str) -> Option<String> {
        self.0.get(key).cloned()
    }
}

/// Generic code can call the method as usual.
async fn lookup<S: Store>(store: &S, key: &str) -> String {
    store
        .get(key)
        .await
        .unwrap_or_else(|| "<missing>".to_owned())
}

// The compiler cannot know that `S::get`'s future is `Send` for every `S`, so
// this does not compile:
//
//     async fn lookup_spawned<S: Store + Send + Sync + 'static>(store: S) {
//         tokio::spawn(async move { lookup(&store, "k").await }).await.ok();
//     }
//     // error: future cannot be sent between threads safely
//     // note: ... the trait `Send` is not implemented for `impl Future<...>`
//
// See `rpitit_send` for the fixes.

/// Calls the generic function on the Tokio runtime.
#[tokio::main]
async fn main() {
    let store = MemoryStore(HashMap::from([("k".to_owned(), "v".to_owned())]));
    assert_eq!(lookup(&store, "k").await, "v");
    assert_eq!(lookup(&store, "nope").await, "<missing>");
}
// ANCHOR_END: example

/// Runs the example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
