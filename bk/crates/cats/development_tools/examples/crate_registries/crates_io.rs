#![allow(dead_code)]
// ANCHOR: example
//! Examples of use of the 'crates.io' API.

use anyhow::Result;
use crates_io_api::CrateResponse;
use crates_io_api::CratesPage;
use crates_io_api::Sort;
use crates_io_api::SyncClient;

/// Instantiate the 'crates.io' API client.
/// The client is configured with a rate limit.
#[tracing::instrument(err)]
fn get_client() -> Result<SyncClient> {
    let client = SyncClient::new(
        "my-user-agent (my-contact@domain.com)",
        std::time::Duration::from_millis(1000),
    )?;
    Ok(client)
}

/// Retrieve information for a given crate.
#[tracing::instrument(err)]
fn get_info_for_crate(crate_name: &str) -> Result<CrateResponse> {
    let client = get_client()?;
    println!("Calling the 'crates.io' API for {crate_name}");
    let crt = client.get_crate(crate_name)?;
    Ok(crt)
}

/// List the top dependencies for the most downloaded crates.
#[tracing::instrument(err)]
fn list_top_dependencies() -> Result<()> {
    let client = get_client()?;
    // Retrieve summary data.
    let summary = client.summary()?;
    for c in summary.most_downloaded {
        tracing::info!("{} - {}:", c.id, c.downloads);
        for dep in client.crate_dependencies(&c.id, &c.max_version)? {
            // Ignore optional dependencies.
            if !dep.optional {
                tracing::info!("    * {}", dep.crate_id);
            }
        }
    }
    Ok(())
}

/// Search for crates matching a given query.
#[tracing::instrument(skip(search), err)]
fn search_for_crates(search: impl Into<String>) -> Result<CratesPage> {
    let client = get_client()?;

    // Build a query first:
    let q = crates_io_api::CratesQuery::builder()
        .sort(Sort::Relevance)
        .page_size(1) // To limit the output size of this example
        .search(search)
        // You can also specify `.category("category-slug")`.
        .build();
    // Retrieve a page of crates:
    let crates = client.crates(q)?;
    Ok(crates)
}

#[tracing::instrument(err)]
fn main() -> Result<()> {
    // Initialize tracing subscriber
    tracing_subscriber::fmt::try_init()
        .map_err(|e| anyhow::anyhow!(e.to_string()))?;
    tracing::info!("Starting crates_io API example");

    // Search for crates that mention "signature verification":
    tracing::info!("Search:\n");
    let result = search_for_crates("signature verification")?;
    tracing::info!("{result:#?}");

    // Display information for a crate:
    tracing::info!("\n\nInfo:\n");
    let info = get_info_for_crate("wgsldoc")?;
    tracing::info!("{info:#?}");

    tracing::info!("\n\nTop dependencies for the most downloaded crates:\n");
    list_top_dependencies()?;

    tracing::info!("crates_io API example completed successfully.");

    Ok(())
}

// ANCHOR_END: example

pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[ignore = "requires crates.io API access"]
    fn test() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }
}
