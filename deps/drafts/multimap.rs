use multimap::MultiMap;
use std::hash::RandomState;
use anyhow::Result;
use crates_io_api::SyncClient;
use crates_io_api::Category;
use itertools::Itertools;

fn get_categories_for_crate(crate_name: &str) -> Result<Vec<Category>> {
    // Instantiate the crates.io API client.
        let client = SyncClient::new(
        "my-user-agent (my-contact@domain.com)",
        std::time::Duration::from_millis(1000), // rate limit interval
    )?;
    let crt = client.get_crate(crate_name)?;
    //println!("Categories: {:?}", crt.categories);
    Ok(crt.categories)
}

#[test]
fn test() {
    let names = vec!["clap", "toml", "config"];
    let crates_with_categories: Result<Vec<(String, Vec<Category>)>> =
                i.names
                    .into_iter()
                    .map(|name| {
                        let cats = get_categories_for_crate(&n)?;
                        Ok((name, cats))
                    })
                    .collect(); // also converts Vec<Result<_>> into Result<Vec<_>>

    // Flatten to a list of tuples, putting the category first
    let category_and_crate: Vec<(Category, String)> = crates_with_categories?
              .into_iter().flat_map(|(n, cs)| { cs.into_iter().map(|cat| (cat.slug, n) )});

    // There are multiple crates for the same category
    // Load into a multimap, which allows multiple values for the same key
    let mm: MultiMap<String, Category, RandomState> = MultiMap::from_iter(crates_with_categories?.into_iter());

    // get all values for a given key
    println!("{}", mm.get_vec("configuration"));

    // or iterate over all keys and the key's vector.
    for (key, values) in mm.iter_all() {
        println!("key: {:?}, values: {:?}", key, values);
    }
}
