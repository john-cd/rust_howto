// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Example using Facebook's `RocksDB` embeddable database.
//!
//! Ensure you have the '`RocksDB`' C++ library installed on your system,
//! because the Rust `rocksdb` crate is just a wrapper around the native C++
//! `RocksDB` library. If you're on `Ubuntu`, we can install it with:
//! ```sh
//! sudo apt-get install librocksdb-dev
//! ```
//! On `macOS`, we can install it with:
//! ```sh
//! brew install rocksdb
//! ```
//! You will also need Clang and LLVM.

use rocksdb::DB;
use rocksdb::Options;

/// Opens a database in a temporary directory, then writes and reads values.
fn main() -> anyhow::Result<()> {
    // Create a new temporary directory to store the database,
    // which will be deleted when `tempdir` goes out of scope.
    let tempdir = tempfile::Builder::new()
        .prefix("rocksdb_storage")
        .tempdir()
        .expect(
            "Failed to create the temporary directory for `rocksdb` storage",
        );
    let path = tempdir.path();
    // In real life, use e.g.: `let path = "my_rocksdb_path";`.
    {
        // Open a RocksDB database.
        // This will create a new database if it doesn't exist:
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

pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the rocksdb example through its entry point.
    #[test]
    fn test_rocksdb_example_entry_point() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }

    #[test]
    fn put_get_and_delete_round_trip() -> anyhow::Result<()> {
        let dir = tempfile::tempdir()?;
        let db = DB::open_default(dir.path())?;
        assert_eq!(db.get(b"key1")?, None);
        db.put(b"key1", b"value1")?;
        assert_eq!(db.get(b"key1")?.as_deref(), Some(&b"value1"[..]));
        db.put(b"key1", b"value2")?;
        assert_eq!(db.get(b"key1")?.as_deref(), Some(&b"value2"[..]));
        db.delete(b"key1")?;
        assert_eq!(db.get(b"key1")?, None);
        // Deleting a missing key is not an error.
        db.delete(b"never-written")?;
        Ok(())
    }

    #[test]
    fn data_survives_reopening_the_database() -> anyhow::Result<()> {
        let dir = tempfile::tempdir()?;
        {
            let db = DB::open_default(dir.path())?;
            db.put(b"k", b"v")?;
        }
        let db = DB::open_default(dir.path())?;
        assert_eq!(db.get(b"k")?.as_deref(), Some(&b"v"[..]));
        Ok(())
    }

    #[test]
    fn read_only_open_of_a_missing_database_fails() {
        let dir = tempfile::tempdir().unwrap();
        let missing = dir.path().join("missing");
        assert!(
            DB::open_for_read_only(&Options::default(), missing, false)
                .is_err()
        );
    }
}
