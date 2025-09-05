#![allow(dead_code)]
// ANCHOR: example
// COMING SOON
// ANCHOR_END: example
//! # AWS SDK Example
//!
//! This example demonstrates how to use the AWS SDK for Rust to interact with
//! Amazon S3.
//!
//! It lists the contents of a specified S3 bucket.

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
    let _region_provider =
        RegionProviderChain::default_provider().or_else("us-west-2");
    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    // FIXME review .region(region_provider);
    let client = Client::new(&config);

    // List objects in the S3 bucket.
    let bucket_name = "your-bucket-name";
    let result = client.list_objects_v2().bucket(bucket_name).send().await;

    match result {
        Ok(output) => {
            if let Some(objects) = output.contents {
                for object in objects {
                    info!("Object key: {}", object.key.unwrap_or_default());
                }
            }
        }
        Err(e) => {
            error!("Failed to list objects: {e}");
        }
    }

    Ok(())
}

#[test]
fn require_network() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
// [finish](https://github.com/john-cd/rust_howto/issues/879)
