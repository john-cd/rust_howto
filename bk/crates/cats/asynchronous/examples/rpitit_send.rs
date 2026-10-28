// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Makes the futures of a trait's async methods `Send`.

use std::future::Future;

/// A store whose futures are `Send`, written with return-position
/// `impl Trait` in a trait (RPITIT).
trait Refresh {
    /// Refreshes the store; the returned future is `Send`.
    fn refresh(&mut self) -> impl Future<Output = Result<u32, String>> + Send;
}

/// A counter that "refreshes" by incrementing.
struct Counter(u32);

impl Refresh for Counter {
    // An `async fn` may implement an `impl Future + Send` method as long as
    // its future really is `Send`.
    async fn refresh(&mut self) -> Result<u32, String> {
        self.0 += 1;
        Ok(self.0)
    }
}

/// Spawns the refresh on a multi-threaded runtime: this requires `Send`.
async fn refresh_in_task<R: Refresh + Send + 'static>(
    mut r: R,
) -> Result<u32, String> {
    tokio::spawn(async move { r.refresh().await })
        .await
        .map_err(|e| e.to_string())?
}

/// The same trait, with `trait_variant` generating a `Send` flavor.
#[trait_variant::make(SendStore: Send)]
trait LocalStore {
    /// Returns the value for `key`.
    async fn get(&self, key: &str) -> Option<String>;
}

/// A [`SendStore`] backed by a fixed value.
struct Fixed;

impl SendStore for Fixed {
    async fn get(&self, key: &str) -> Option<String> {
        Some(format!("value of {key}"))
    }
}

/// Spawns a [`SendStore`] lookup.
async fn get_in_task<S: SendStore + Send + Sync + 'static>(
    s: S,
) -> Option<String> {
    tokio::spawn(async move { s.get("k").await })
        .await
        .ok()
        .flatten()
}

/// Runs both `Send` future examples and prints their results.
async fn example() {
    println!("{:?}", refresh_in_task(Counter(0)).await);
    println!("{:?}", get_in_task(Fixed).await);
}

/// Runs the example on Tokio's default runtime.
#[tokio::main]
async fn main() {
    example().await;
}

// ANCHOR_END: example

/// Runs the example on a multi-threaded Tokio runtime.
///
/// # Errors
///
/// Returns an error if the Tokio runtime cannot be created.
pub fn run() -> anyhow::Result<()> {
    let runtime = tokio::runtime::Builder::new_multi_thread()
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

    /// Verifies both styles run on a multi-threaded runtime.
    #[tokio::test(flavor = "multi_thread")]
    async fn futures_are_send() {
        assert_eq!(refresh_in_task(Counter(0)).await, Ok(1));
        assert_eq!(get_in_task(Fixed).await.as_deref(), Some("value of k"));
    }
}
