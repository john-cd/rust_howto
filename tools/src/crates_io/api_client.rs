use anyhow::Result;
use crates_io_api::CrateResponse;
use crates_io_api::SyncClient;
use tracing::warn;

use super::cat::Cat;

/// Instantiate the crates.io API client.
fn get_client() -> Result<SyncClient> {
    let client = SyncClient::new(
        "my-user-agent (my-contact@domain.com)",
        std::time::Duration::from_millis(1000), // rate limit interval
    )?;
    Ok(client)
}

/// Returns information for a crate, given its name
pub fn get_info_for_crate(crate_name: &str) -> Result<CrateResponse> {
    let client = get_client()?;
    warn!("Calling crates.io API for {crate_name}");
    let crt = client.get_crate(crate_name)?;
    Ok(crt)
}

/// Returns a list of categories for a crate, given its name
pub fn get_categories_for_crate(crate_name: &str) -> Result<Vec<Cat>> {
    let client = get_client()?;
    warn!("Calling crates.io API for {crate_name}");
    let crt = client.get_crate(crate_name)?;
    // println!("Categories: {:?}", crt.categories);
    Ok(crt.categories.into_iter().map(Cat::new).collect())
}

// /// List, for most downloaded crates, the top dependencies
// pub fn list_top_dependencies() -> Result<()> {
//     let client = get_client()?;
//     // Retrieve summary data.
//     let summary = client.summary()?;
//     for c in summary.most_downloaded {
//         println!("{}:", c.id);
//         for dep in client.crate_dependencies(&c.id, &c.max_version)? {
//             // Ignore optional dependencies.
//             if !dep.optional {
//                 println!("    * {} - {}", dep.id, dep.version_id);
//             }
//         }
//     }
//     Ok(())
// }

// use crates_io_api::CratesPage;
// use crates_io_api::Sort;
// /// Search for crates
// pub fn search_for_crates(search: impl Into<String>) -> Result<CratesPage> {
//     let client = get_client()?;
//
//     let q = crates_io_api::CratesQuery::builder()
//     .sort(Sort::Alphabetical)
//     //.page_size(size)
//     .search(search)
//     .build();
//     let crates = client.crates(q)?;
//     Ok(crates)
// }
