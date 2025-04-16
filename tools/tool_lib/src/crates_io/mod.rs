//! Returns information / list of categories for a given crate, using the
//! crates.io API

mod categories_for_crate;
mod info;

use anyhow::Result;
pub use categories_for_crate::*;
use crates_io_api::SyncClient;
pub use info::*;

/// Instantiate the crates.io API client.
fn get_client() -> Result<SyncClient> {
    let client = SyncClient::new(
        "my-user-agent (my-contact@domain.com)",
        std::time::Duration::from_millis(1000), // rate limit interval
    )?;
    Ok(client)
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
// [unit tests; cleanup](https://github.com/john-cd/rust_howto/issues/1356)
