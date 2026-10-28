// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! A native `async fn` in a trait, called through generics.

/// A key-value store with an asynchronous lookup.
trait Store {
    /// Returns the value stored under `key`, if any.
    async fn get(&self, key: &str) -> Option<String>;
}

/// An in-memory [`Store`].
struct MemoryStore(Vec<(String, String)>);

impl Store for MemoryStore {
    async fn get(&self, key: &str) -> Option<String> {
        self.0
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.clone())
    }
}

/// Looks up `key` through any [`Store`]; the compiler monomorphizes the call.
async fn lookup<S: Store>(store: &S, key: &str) -> String {
    store.get(key).await.unwrap_or_else(|| "<missing>".into())
}

// The following does NOT compile: the future returned by `Store::get` is not
// known to be `Send`, so it cannot be moved onto a multi-threaded executor.
//
//     fn spawn_lookup<S: Store + Send + Sync + 'static>(store: S) {
//         tokio::spawn(async move { lookup(&store, "a").await });
//     }
//
// error: future cannot be sent between threads safely
//   = note: the trait `Send` is not implemented for
//           `impl Future<Output = Option<String>>`
// See `rpitit_send.rs` for the fix.

/// Looks up two keys and prints their values.
async fn example() {
    let store = MemoryStore(vec![("a".into(), "1".into())]);
    println!("a = {}", lookup(&store, "a").await);
    println!("b = {}", lookup(&store, "b").await);
}

/// Runs the example on a current-thread Tokio runtime.
#[tokio::main(flavor = "current_thread")]
async fn main() {
    example().await;
}

// ANCHOR_END: example

/// Runs the example on a current-thread Tokio runtime.
///
/// # Errors
///
/// Returns an error if the Tokio runtime cannot be created.
pub fn run() -> anyhow::Result<()> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;
    runtime.block_on(example());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies the example entry point runs successfully.
    #[test]
    fn main_runs() -> anyhow::Result<()> {
        run()
    }

    /// Verifies lookups of present and missing keys.
    #[tokio::test]
    async fn lookup_works() {
        let store = MemoryStore(vec![("a".into(), "1".into())]);
        assert_eq!(lookup(&store, "a").await, "1");
        assert_eq!(lookup(&store, "b").await, "<missing>");
    }
}
