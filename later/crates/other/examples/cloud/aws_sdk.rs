#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! This example demonstrates how to use the AWS SDK for Rust to list Amazon S3
//! buckets visible to the configured AWS identity.

use aws_config::BehaviorVersion;
use aws_sdk_s3::Client;
use tracing::info;

/// Lists the S3 buckets visible to the configured AWS identity.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing subscriber.
    tracing_subscriber::fmt::init();

    let config = aws_config::defaults(BehaviorVersion::latest()).load().await;
    let client = Client::new(&config);

    let output = client.list_buckets().send().await?;
    for bucket in output.buckets() {
        if let Some(name) = bucket.name() {
            info!(bucket = %name, "S3 bucket");
        }
    }

    Ok(())
}
// ANCHOR_END: example

pub fn run() -> anyhow::Result<()> {
    main()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Requires AWS credentials and network access, so it is ignored by
    /// default.
    #[test]
    #[ignore = "needs AWS credentials and network access"]
    fn list_s3_buckets() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }
}
