#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Demonstrates the use of a global mutable state protected by a `Mutex`.
//!
//! `LazyLock` initializes a global `Mutex` containing a `Vec<String>` once.

use std::sync::LazyLock;
use std::sync::Mutex;

use anyhow::Result;
use anyhow::anyhow;

/// Global fruit list initialized once and protected by a mutex.
static FRUIT: LazyLock<Mutex<Vec<String>>> =
    LazyLock::new(|| Mutex::new(Vec::new()));

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

pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies inserts are visible through the synchronized global list.
    /// Runs the global mut state example through its entry point.
    #[test]
    fn test_global_mut_state_example_entry_point() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }
}
