#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! This function demonstrates basic usage of the `heed` crate for interacting
//! with an LMDB database. It covers opening a database, creating a write
//! transaction, writing data, committing the transaction, opening a read
//! transaction, and reading data.

// This example demonstrates using the `heed` crate as an LMDB-backed key-value
// store.
use heed::CommitOrAbort;
use heed::Database;
use heed::EnvOpenOptions;
use heed::types::*;

// `heed` (and `heed3`) are high-level wrappers of LMDB.
// Lightning Memory-Mapped Database (LMDB) is an embedded, transactional
// database in the form of a key-value store.

/// This function demonstrates basic usage of the `heed` crate for interacting
/// with an LMDB database. It covers opening a database, creating a write
/// transaction, writing data, committing the transaction, opening a read
/// transaction, and reading data.
fn main() -> anyhow::Result<()> {
    // Open a database:
    let dir =
        tempfile::tempdir().expect("the heed example invariant should hold");
    let env = unsafe {
        EnvOpenOptions::new()
            .map_size(1024 * 1024 * 10) // 10 MiB
            .max_dbs(10)
            .open(dir.path())?
    };

    // Create the default unnamed database inside a write transaction, which
    // is committed along with the database. We specify the key type as a
    // 32-bit unsigned integer in native endianness and the value type as a
    // string.
    let wtxn = env.write_txn()?;
    let db: Database<U32<byteorder::NativeEndian>, Str> = env
        .create_databases_and_commit(wtxn, |opener, wtxn| {
            let db = opener.create_database(wtxn, None)?;
            anyhow::Ok(CommitOrAbort::<_, ()>::Commit(db))
        })?
        .unwrap_commit();

    // Create a transaction with read and write access for use with the
    // environment.
    let mut wtxn = env.write_txn()?;

    // Write data to the database. We insert a key-value pair where the key is
    // 1 and the value is "Hello, world!".
    db.put(&mut wtxn, &1, "Hello, world!")?;
    // Commit the write transaction to persist the changes.
    wtxn.commit()?;

    // Open a read transaction.
    let rtxn = env.read_txn()?;
    // Read data from the database:
    let value = db
        .get(&rtxn, &1)?
        .expect("expected example value should be present");
    println!("Value: {value}");
    assert_eq!(value, "Hello, world!");

    Ok(())
}

// ANCHOR_END: example

pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the heed example through its entry point.
    #[test]
    fn test_heed_example_entry_point() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }
}
