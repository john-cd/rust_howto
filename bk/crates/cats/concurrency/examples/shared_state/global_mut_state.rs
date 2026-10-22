#![allow(dead_code)]
// ANCHOR: example
//! Demonstrates the use of a global mutable state protected by a `Mutex`.
//!
//! This example uses `lazy_static` to initialize a global `Mutex` containing a
//! `Vec<String>`.
//!
//! TODO Reviewed ??-??-2026

use std::sync::Mutex;

use anyhow::Result;
use anyhow::anyhow;
use lazy_static::lazy_static;

lazy_static! {
    // A global `Mutex` containing a vector of strings representing fruits.
    static ref FRUIT: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

/// Adds `fruit` to the global database.
fn insert(fruit: &str) -> Result<()> {
    let mut db = FRUIT
        .lock()
        .map_err(|_| anyhow!("Failed to acquire MutexGuard"))?;
    db.push(fruit.to_owned());
    Ok(())
}

/// Adds fruits to the global database from the main thread and prints its
/// content.
fn main() -> Result<()> {
    insert("apple")?;
    insert("orange")?;
    insert("peach")?;
    {
        let db = FRUIT
            .lock()
            .map_err(|_| anyhow!("Failed to acquire MutexGuard"))?;

        db.iter().enumerate().for_each(|(i, item)| {
            println!("{i}: {item}");
        });
    }
    insert("grape")?;
    Ok(())
}

// ANCHOR_END: example

/// Runs the example.
pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }
}
// [review lazy_constant.rs and lazy_static.rs and global_mut_state.rs](https://github.com/john-cd/rust_howto/issues/939)
