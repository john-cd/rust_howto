#![allow(dead_code)]
// ANCHOR: example
//! Example using Facebook's RocksDB embeddable database.
//!
//! Ensure you have the RocksDB C++ library installed on your system, because
//! the Rust `rocksdb` crate is just a wrapper around the native C++ RocksDB
//! library. If you're on `Ubuntu`, you can install it with:
//! ```sh
//! sudo apt-get install librocksdb-dev
//! ```
//! On `macOS`, you can install it with:
//! ```sh
//! brew install rocksdb
//! ```
//! You will also need Clang and LLVM.

use rocksdb::DB;
use rocksdb::Options;

fn main() -> anyhow::Result<()> {
    // Create a new temporary directory to store the database,
    // which will be deleted when `tempdir` goes out of scope
    let tempdir = tempfile::Builder::new()
        .prefix("rocksdb_storage")
        .tempdir()
        .expect(
            "Failed to create the temporary directory for `rocksdb` storage",
        );
    let path = tempdir.path();
    // In real life, use e.g.: let path = "my_rocksdb_path";
    {
        // Open a RocksDB database.
        // This will create a new database if it doesn't exist.
        let db = DB::open_default(path)?;

        // Insert some key-value pairs:
        db.put(b"key1", b"value1")?;
        db.put(b"key2", b"value2")?;

        // Retrieve a value by its key:
        match db.get(b"key1")? {
            Some(value) => println!(
                "Found key1 with value: {}",
                String::from_utf8_lossy(&value)
            ),
            None => println!("key1 not found"),
        }

        // Delete a key-value pair:
        db.delete(b"key1")?;

        // Try to get the deleted key:
        match db.get(b"key1")? {
            Some(value) => println!(
                "Found key1 with value: {}",
                String::from_utf8_lossy(&value)
            ),
            None => println!("key1 not found after deletion"),
        }
    }
    let _ = DB::destroy(&Options::default(), path);

    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
