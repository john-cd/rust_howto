#![allow(dead_code)]
// ANCHOR: example
//! This example demonstrates how to use a `MultiMap` to store multiple values
//! for the same key.
use anyhow::Result;
use crates_io_api::Category;
use crates_io_api::SyncClient;
use multimap::MultiMap;

// Calls the crates.io API client and retrieve the categories a given crate
// belongs to.
fn get_categories_for_crate(crate_name: &str) -> Result<Vec<Category>> {
    let client = SyncClient::new(
        "my-user-agent (my-contact@domain.com)",
        std::time::Duration::from_millis(1000), // Rate limit interval
    )?;
    // Retrieve the crate's information:
    let crt = client.get_crate(crate_name)?;
    Ok(crt.categories)
}

fn main() -> Result<()> {
    let crate_names = vec!["toml", "config", "nom", "pest"];

    let mut m: MultiMap<String, &str> = MultiMap::new();
    for name in crate_names {
        for cat in get_categories_for_crate(name)? {
            // There can be multiple crates in the same category.
            // A multimap allows multiple values for the same key.
            m.insert(cat.slug, name);
        }
    }

    // Get all values for a given key:
    println!(
        "List of crates in the `config` category: {:?}",
        m.get_vec("config")
    );

    // Or iterate over all keys and the key's vector:
    for (cat, names) in m.iter_all() {
        println!("Category: {cat:?}, names: {names:?}");
    }
    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
