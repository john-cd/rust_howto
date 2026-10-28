// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Example using the `redb` embedded key-value store.
//!
//! `redb` is a pure-Rust, ACID-compliant embedded key-value database with a
//! stable file format.
//!
//! Add `redb = "2.4.0"` to `Cargo.toml`.

use redb::Database;
use redb::TableDefinition;

const TABLE: TableDefinition<&str, &str> = TableDefinition::new("my_table");

/// Opens a `redb` database, inserts a key-value pair in a write transaction,
/// and reads it back in a read transaction.
fn main() -> anyhow::Result<()> {
    // Create a temporary directory for the database file:
    let dir = tempfile::tempdir()?;
    let db_path = dir.path().join("my_db.redb");

    // Create and open the database:
    let db = Database::create(db_path)?;

    // Perform a write transaction:
    let write_txn = db.begin_write()?;
    {
        let mut table = write_txn.open_table(TABLE)?;
        table.insert("key1", "value1")?;
    }
    write_txn.commit()?;

    // Perform a read transaction:
    let read_txn = db.begin_read()?;
    let table = read_txn.open_table(TABLE)?;
    let value = table.get("key1")?;
    assert_eq!(
        value.map(|v| v.value().to_string()),
        Some("value1".to_string())
    );

    print!("`redb` example ran successfully.");

    Ok(())
}
// ANCHOR_END: example

pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the redb example and verifies key-value insertion and retrieval.
    #[test]
    fn test_redb_example_entry_point() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }
}
