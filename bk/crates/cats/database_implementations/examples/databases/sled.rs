#![allow(dead_code)]
// ANCHOR: example
//! Example using the `sled` embeddable database.
//!
//! `sled` is a pure-Rust high-performance embedded database.
//!
//! Add `sled = "0.34.7"` (or latest) to `Cargo.toml`.

fn main() -> anyhow::Result<()> {
    // Open a sled database.
    // A directory will be created if it does not exist:
    let db: sled::Db = sled::open("temp/my_db")?;

    // Insert a key to a new value, returning the last value if it was set:
    assert_eq!(db.insert(b"key1", b"Hello, Sled!"), Ok(None));

    // Retrieve a value, if it exists:
    assert_eq!(&db.get(b"key1")?.unwrap(), b"Hello, Sled!");
    // Note: under the cover, `get` returns an `IVec`, which is a data structure
    // that makes some things more efficient. IVec implements `AsRef<[u8]>`,
    // so we can use `&` above.
    assert_eq!(db.get(b"key1")?, Some(sled::IVec::from(b"Hello, Sled!")));

    // `key2` does not exist:
    assert_eq!(db.get(b"key2")?, None);

    // `sled` is fully thread-safe, and all operations are atomic.

    // Atomic compare-and-swap, capable of unique creation, conditional
    // modification, or deletion:
    // - If old is `None`, it will only set the value if it doesn't exist yet.
    // - If new is `None`, it will delete the value if old is correct.
    // - If both old and new are `Some`, it will modify the value only if old is
    //   correct.
    db.compare_and_swap(
        b"key1",               // key
        Some(b"Hello, Sled!"), // old value
        Some(b"Hey"),          // new value, None for delete
    )??;

    // Remove the key-value pair:
    let old_value = db.remove(b"key1")?;
    assert_eq!(old_value, Some(sled::IVec::from(b"Hey")),);
    assert_eq!(db.get(b"key1"), Ok(None));

    // Perform a multi-key serializable transaction:
    // for example, use write-only transactions as a writebatch:
    db.transaction::<_, _, sled::Error>(|tx_db| {
        tx_db.insert(b"key3", b"stuff")?;
        tx_db.insert(b"key4", b"much")?;
        Ok(())
    })?;

    assert_eq!(&db.get(b"key3")?.unwrap(), b"stuff");
    assert_eq!(&db.get(b"key4")?.unwrap(), b"much");

    // Multiple trees with isolated keyspaces are supported with the
    // `Db::open_tree` method.
    let other_tree: sled::Tree = db.open_tree(b"another tree")?;
    other_tree.insert(b"k1", &b"value"[..])?;
    db.drop_tree(b"another tree")?;

    // Synchronously flushes all dirty IO buffers and calls fsync.
    // If this succeeds, it is guaranteed that all previous writes will be
    // recovered if the system crashes. Returns the number of bytes flushed
    // during this call. `flush_async` is also available.
    db.flush()?;

    print!("`sled` example ran successfully.");

    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
