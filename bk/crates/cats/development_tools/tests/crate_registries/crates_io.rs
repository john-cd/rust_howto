// ANCHOR: example
use anyhow::Result;
use crates_io_api::CrateResponse;
use crates_io_api::CratesPage;
use crates_io_api::Sort;
use crates_io_api::SyncClient;
use tracing::warn;

/// Instantiate the 'crates.io' API client.
fn get_client() -> Result<SyncClient> {
    let client = SyncClient::new(
        "my-user-agent (my-contact@domain.com)",
        std::time::Duration::from_millis(1000), // Rate limit interval
    )?;
    Ok(client)
}

/// Returns information for a crate, given its name.
fn get_info_for_crate(crate_name: &str) -> Result<CrateResponse> {
    let client = get_client()?;
    warn!("Calling the 'crates.io' API for {crate_name}");
    let crt = client.get_crate(crate_name)?;
    Ok(crt)
}

/// List, for the most downloaded crates, the top dependencies.
fn list_top_dependencies() -> Result<()> {
    let client = get_client()?;
    // Retrieve summary data.
    let summary = client.summary()?;
    for c in summary.most_downloaded {
        println!("{} - {}:", c.id, c.downloads);
        for dep in client.crate_dependencies(&c.id, &c.max_version)? {
            // Ignore optional dependencies.
            if !dep.optional {
                println!("    * {}", dep.crate_id);
            }
        }
    }
    Ok(())
}

/// Search for crates.
fn search_for_crates(search: impl Into<String>) -> Result<CratesPage> {
    let client = get_client()?;

    // Build a query first
    let q = crates_io_api::CratesQuery::builder()
        .sort(Sort::Relevance)
        .page_size(1) // To limit the output size of this example
        .search(search)
        // You can also specify .category("category-slug")
        .build();
    // Retrieve a page of crates
    let crates = client.crates(q)?;
    Ok(crates)
}

fn main() -> Result<()> {
    // Search for crates that mention "signature verification"
    println!("Search:\n");
    let result = search_for_crates("signature verification")?;
    println!("{:#?}", result);

    // Display information for a crate
    println!("\n\nInfo:\n");
    let info = get_info_for_crate("wgsldoc")?;
    println!("{:#?}", info);

    println!("\n\nTop dependencies for the most downloaded crates:\n");
    list_top_dependencies()?;

    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
