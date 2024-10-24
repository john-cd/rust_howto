use anyhow::Result;
use crates_io_api::SyncClient;

/// Instantiate the crates.io API client.
fn get_client() -> Result<SyncClient> {
    let client = SyncClient::new(
        "my-user-agent (my-contact@domain.com)",
        std::time::Duration::from_millis(1000), // rate limit interval
    )?;
    Ok(client)
}

pub fn list_categories(crate_name: &str) -> Result<()> {
    let client = get_client()?;
    let crt = client.get_crate(crate_name)?;
    // println!("{:?}", crt);
    let cats = crt.categories;
    println!("Categories: {:?}", cats);
    Ok(())
}

pub fn list_top_dependencies() -> Result<()> {
    let client = get_client()?;
    // Retrieve summary data.
    let summary = client.summary()?;
    for c in summary.most_downloaded {
        println!("{}:", c.id);
        for dep in client.crate_dependencies(&c.id, &c.max_version)? {
            // Ignore optional dependencies.
            if !dep.optional {
                println!("    * {} - {}", dep.id, dep.version_id);
            }
        }
    }
    Ok(())
}
