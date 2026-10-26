#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! `#[async_trait]` for trait objects.
//!
//! Native `async fn` in traits are not dyn-compatible: `Box<dyn Store>`
//! does not compile without the macro, which boxes every returned future.

use async_trait::async_trait;

/// A store usable as a trait object.
#[async_trait]
trait Store: Send + Sync {
    /// The name of the store.
    fn name(&self) -> &'static str;
    /// Returns the value stored under `key`, if any.
    async fn get(&self, key: &str) -> Option<String>;
}

/// A store that always answers.
struct Always(&'static str);

/// A store that never has a value.
struct Empty;

#[async_trait]
impl Store for Always {
    fn name(&self) -> &'static str {
        "always"
    }

    async fn get(&self, _key: &str) -> Option<String> {
        Some(self.0.to_owned())
    }
}

#[async_trait]
impl Store for Empty {
    fn name(&self) -> &'static str {
        "empty"
    }

    async fn get(&self, _key: &str) -> Option<String> {
        None
    }
}

/// Returns the first value found by a chain of stores of different types.
async fn first_hit(
    stores: &[Box<dyn Store>],
    key: &str,
) -> Option<(String, String)> {
    for store in stores {
        if let Some(v) = store.get(key).await {
            return Some((store.name().to_owned(), v));
        }
    }
    None
}

/// Runs the trait-object lookups and prints the first matching value.
async fn example() {
    let stores: Vec<Box<dyn Store>> =
        vec![Box::new(Empty), Box::new(Always("hit"))];
    println!("{:?}", first_hit(&stores, "k").await);
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

    /// Verifies the chain skips the empty store.
    #[tokio::test]
    async fn chain_finds_first_hit() {
        let stores: Vec<Box<dyn Store>> =
            vec![Box::new(Empty), Box::new(Always("hit"))];
        assert_eq!(
            first_hit(&stores, "k").await,
            Some(("always".to_owned(), "hit".to_owned()))
        );
        assert_eq!(first_hit(&[Box::new(Empty)], "k").await, None);
    }
}
