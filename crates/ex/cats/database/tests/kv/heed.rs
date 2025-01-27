// // ANCHOR: example

// // `heed` and `heed3` are high-level wrappers of LMDB.
// // Lightning Memory-Mapped Database (LMDB) is an embedded transactional
// // database in the form of a key-value store.

// // `heed` is a wrapper around LMDB on the mdb.master branch,
// // `heed3` derives from the heed wrapper but on the mdb.master3 branch.
// // The heed3 crate will be stable once the LMDB version on the mdb.master3
// // branch will be officially released. It features encryption-at-rest and
// // checksumming features that the heed crate doesn't.

// use heed::types::*;
// use heed::{Database, EnvOpenOptions};

// fn main() -> anyhow::Result<()> {
//     // Open a database

//    let env = unsafe {
//         EnvOpenOptions::new()
//             .map_size(1024 * 1024 * 10) // 10 MiB
//             .max_dbs(10)
//             .open("my_database")?
//     };

//     // Create a database within the environment
//     let db: Database<u32, String> = env.create_database(b"my_database",
// None)?;

//     // Write data to the database
//     let mut wtxn = env.write_txn()?;
//     db.put(&mut wtxn, &1, &"Hello, world!".to_string())?;
//     wtxn.commit()?;

//     // Read data from the database
//     let mut rtxn = env.read_txn()?;
//     let value = db.get(&rtxn, &1)?.unwrap();
//     println!("Value: {}", value);

//     Ok(())
// }
// // ANCHOR_END: example

// #[test]
// fn require_external_svc() -> anyhow::Result<()> {
//     main()?;
//     Ok(())
// }

// [ P2 write heed example](https://github.com/john-cd/rust_howto/issues/1018)
