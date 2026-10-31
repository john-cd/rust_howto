// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! A `MultiMap`, which associates several values with one key.
//!
//! A multimap associates several values with one key, for example the tags of a
//! post or the headers of an HTTP request that may repeat.

use anyhow::Result;
use crates_io_api::Category;
use crates_io_api::SyncClient;
use multimap::MultiMap;

// Calls the crates.io API client and retrieves the categories for a crate.
/// Returns the categories of the crate `crate_name`, as listed on `crates.io`.
fn get_categories_for_crate(crate_name: &str) -> Result<Vec<Category>> {
    let client = SyncClient::new(
        "my-user-agent (my-contact@domain.com)",
        std::time::Duration::from_millis(1000), // Rate limit interval.
    )?;
    // Retrieve the crate's information:
    let crt = client.get_crate(crate_name)?;
    Ok(crt.categories)
}

/// Groups crates by category in a `MultiMap`.
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

pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the multimap example through its entry point.
    #[test]
    #[ignore = "requires external resources"]
    fn test_multimap_example_entry_point() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }
}
