#![allow(dead_code)]
// ANCHOR: example
//! This example demonstrates how to use the AWS SDK for Rust to interact with
//! Amazon S3. It lists the contents of a specified S3 bucket.

use aws_config::BehaviorVersion;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::Client;
use tracing::error;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing subscriber.
    tracing_subscriber::fmt::init();

    // Load AWS configuration.
    let region_provider =
        RegionProviderChain::default_provider().or_else("us-west-2");
    let config = aws_config::defaults(BehaviorVersion::latest())
        .region(region_provider)
        .load()
        .await;
    let client = Client::new(&config);

    // List objects in the S3 bucket.
    let bucket_name = "your-bucket-name";
    let result = client.list_objects_v2().bucket(bucket_name).send().await;

    match result {
        Ok(output) => {
            for object in output.contents() {
                info!("Object key: {}", object.key().unwrap_or_default());
            }
        }
        Err(e) => {
            error!("Failed to list objects: {e}");
        }
    }

    Ok(())
}
// ANCHOR_END: example

#[test]
fn require_network() -> anyhow::Result<()> {
    // main()?; // TODO Skip running S3 queries in simple unit tests to avoid network
    // timeouts / missing AWS credentials.
    Ok(())
}
