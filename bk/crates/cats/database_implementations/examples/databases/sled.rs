#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Example using the `sled` embeddable database.
//!
//! `sled` is a pure-Rust high-performance embedded database.
//!
//! Add `sled = "0.34.7"` (or latest) to `Cargo.toml`.

/// Opens a `sled` database at the specified path.
fn open_db(path: &str) -> anyhow::Result<sled::Db> {
    let db = sled::open(path)?;
    Ok(db)
}

/// Performs basic key-value insertion, retrieval, compare-and-swap, and removal.
fn perform_kv_operations(db: &sled::Db) -> anyhow::Result<()> {
    assert_eq!(db.insert(b"key1", b"Hello, Sled!"), Ok(None));

    assert_eq!(
        &db.get(b"key1")?
            .expect("expected example value should be present"),
        b"Hello, Sled!"
    );
    assert_eq!(db.get(b"key1")?, Some(sled::IVec::from(b"Hello, Sled!")));
    assert_eq!(db.get(b"key2")?, None);

    db.compare_and_swap(
        b"key1",
        Some(b"Hello, Sled!"),
        Some(b"Hey"),
    )??;

    let old_value = db.remove(b"key1")?;
    assert_eq!(old_value, Some(sled::IVec::from(b"Hey")));
    assert_eq!(db.get(b"key1"), Ok(None));
    Ok(())
}

/// Performs transactions and isolated tree operations on a sled database.
fn perform_transactions_and_trees(db: &sled::Db) -> anyhow::Result<()> {
    db.transaction::<_, _, sled::Error>(|tx_db| {
        tx_db.insert(b"key3", b"stuff")?;
        tx_db.insert(b"key4", b"much")?;
        Ok(())
    })?;

    assert_eq!(
        &db.get(b"key3")?
            .expect("expected example value should be present"),
        b"stuff"
    );
    assert_eq!(
        &db.get(b"key4")?
            .expect("expected example value should be present"),
        b"much"
    );

    let other_tree: sled::Tree = db.open_tree(b"another tree")?;
    other_tree.insert(b"k1", &b"value"[..])?;
    db.drop_tree(b"another tree")?;

    db.flush()?;
    Ok(())
}

/// Opens a `sled` database, then inserts, reads and removes values.
fn main() -> anyhow::Result<()> {
    let db = open_db("temp/my_db")?;
    perform_kv_operations(&db)?;
    perform_transactions_and_trees(&db)?;
    print!("`sled` example ran successfully.");
    Ok(())
}

// ANCHOR_END: example

pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the sled example through its entry point.
    #[test]
    fn test_sled_example_entry_point() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }

    #[test]
    fn test_kv_operations() -> anyhow::Result<()> {
        let db = sled::Config::new().temporary(true).open()?;
        perform_kv_operations(&db)?;
        Ok(())
    }

    #[test]
    fn test_transactions_and_trees() -> anyhow::Result<()> {
        let db = sled::Config::new().temporary(true).open()?;
        perform_transactions_and_trees(&db)?;
        Ok(())
    }
}
