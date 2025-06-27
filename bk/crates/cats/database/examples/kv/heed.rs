#![allow(dead_code)]
// ANCHOR: example
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
    let dir = tempfile::tempdir().unwrap();
    let env = unsafe {
        EnvOpenOptions::new()
            .map_size(1024 * 1024 * 10) // 10 MiB
            .max_dbs(10)
            .open(dir.path())?
    };

    // Create a transaction with read and write access for use with the
    // environment.
    let mut wtxn = env.write_txn()?;

    // Open the default unnamed database. We specify the key type as a
    // 32-bit unsigned integer in native endianness and the value type as a
    // string.
    let db: Database<U32<byteorder::NativeEndian>, Str> =
        env.create_database(&mut wtxn, None)?;

    // Write data to the database. We insert a key-value pair where the key is
    // 1 and the value is "Hello, world!".
    db.put(&mut wtxn, &1, "Hello, world!")?;
    // Commit the write transaction to persist the changes.
    wtxn.commit()?;

    // Open a read transaction.
    let rtxn = env.read_txn()?;
    // Read data from the database
    let value = db.get(&rtxn, &1)?.unwrap();
    println!("Value: {value}");
    assert_eq!(value, "Hello, world!");

    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
