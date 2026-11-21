// TODO Reviewed ??-??-2026.
//! Writes, lists, and retrieves objects with interchangeable storage backends.

// ANCHOR: example
use futures_util::TryStreamExt;
use object_store::ObjectStore;
use object_store::ObjectStoreExt;
use object_store::local::LocalFileSystem;
use object_store::path::Path;

/// Writes one object, verifies listing by prefix, and fetches its bytes.
///
/// # Errors
///
/// Returns an error if the backend cannot write, list, or fetch the object.
async fn round_trip(store: &dyn ObjectStore) -> anyhow::Result<()> {
    let prefix = Path::from("reports");
    let key = prefix.clone().join("today.txt");
    store.put(&key, "ready".into()).await?;
    let objects: Vec<_> = store.list(Some(&prefix)).try_collect().await?;
    assert_eq!(objects.len(), 1);
    assert_eq!(objects[0].location, key);
    assert_eq!(store.get(&key).await?.bytes().await?.as_ref(), b"ready");
    Ok(())
}

/// Uses a temporary directory as the local object store root.
#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let directory = tempfile::tempdir()?;
    round_trip(&LocalFileSystem::new_with_prefix(directory.path())?).await
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    /// Exercises the same object operations with memory and filesystem stores.
    #[test]
    fn supports_both_stores() -> anyhow::Result<()> {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?
            .block_on(super::round_trip(&object_store::memory::InMemory::new()))?;
        super::main()
    }
}
