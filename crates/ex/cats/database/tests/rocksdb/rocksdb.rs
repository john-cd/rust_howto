// ANCHOR: example

// Rust wrapper for Facebook's RocksDB embeddable database

// Ensure you have the RocksDB C++ library installed on your system, because the
// Rust rocksdb crate is a wrapper around the native C++ RocksDB library.
// If you're on Ubuntu, you can install it with:
// sudo apt-get install librocksdb-dev
// Requirements: Clang and LLVM

use rocksdb::DB;
// use rocksdb::Error;
// use rocksdb::Options;

fn main() -> anyhow::Result<()> {
    // Open a RocksDB database
    // (this will create a new database if it doesn't exist)
    let path = "my_rocksdb";
    let db = DB::open_default(path)?;

    // Insert some key-value pairs
    db.put(b"key1", b"value1")?;
    db.put(b"key2", b"value2")?;

    // Retrieve a value by its key
    match db.get(b"key1")? {
        Some(value) => println!(
            "Found key1 with value: {}",
            String::from_utf8_lossy(&value)
        ),
        None => println!("key1 not found"),
    }

    // Delete a key-value pair
    db.delete(b"key1")?;

    // Try to get the deleted key
    match db.get(b"key1")? {
        Some(value) => println!(
            "Found key1 with value: {}",
            String::from_utf8_lossy(&value)
        ),
        None => println!("key1 not found after deletion"),
    }

    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
// TODO P2 write
