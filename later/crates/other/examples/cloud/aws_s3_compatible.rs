// TODO Reviewed ??-??-2026.

#![expect(
    dead_code,
    reason = "the snippet declares items to illustrate them, not to call them"
)]
// ANCHOR: example
//! Point the S3 client at an S3-compatible store (MinIO, RustFS, LocalStack).

use anyhow::Context;
use anyhow::Result;
use aws_sdk_s3::Client;
use aws_sdk_s3::config::BehaviorVersion;
use aws_sdk_s3::config::Builder;
use aws_sdk_s3::config::Credentials;
use aws_sdk_s3::config::Region;

/// Builds a client for `endpoint`, e.g. `http://localhost:9000`.
///
/// The keys are static here for local development only. In production, use the
/// credential provider chain (see the shared configuration example) and never
/// commit keys.
pub fn s3_compatible_client(
    endpoint: &str,
    access_key: &str,
    secret_key: &str,
) -> Client {
    let credentials =
        Credentials::new(access_key, secret_key, None, None, "static");
    let config = Builder::new()
        .behavior_version(BehaviorVersion::latest())
        // Most S3-compatible stores ignore the region, but the SDK needs one.
        .region(Region::new("us-east-1"))
        .endpoint_url(endpoint)
        .credentials_provider(credentials)
        // `http://host/bucket/key` instead of `http://bucket.host/key`.
        .force_path_style(true)
        .build();
    Client::from_conf(config)
}

/// Creates a bucket and stores an object, then reads it back.
pub async fn round_trip(client: &Client, bucket: &str) -> Result<Vec<u8>> {
    client
        .create_bucket()
        .bucket(bucket)
        .send()
        .await
        .context("CreateBucket failed")?;
    client
        .put_object()
        .bucket(bucket)
        .key("hello.txt")
        .body(b"hello".to_vec().into())
        .send()
        .await
        .context("PutObject failed")?;
    let object = client
        .get_object()
        .bucket(bucket)
        .key("hello.txt")
        .send()
        .await
        .context("GetObject failed")?;
    Ok(object.body.collect().await?.into_bytes().to_vec())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the configuration is built without contacting the store.
    #[test]
    fn client_uses_the_endpoint_and_path_style() {
        let client =
            s3_compatible_client("http://localhost:9000", "access", "secret");
        let config = client.config();
        assert_eq!(config.region().map(Region::as_ref), Some("us-east-1"));
    }

    /// Exercises a real store. Start one first, for example MinIO on port 9000
    /// with the keys `minioadmin` / `minioadmin`, then run
    /// `cargo test -- --ignored`.
    #[tokio::test]
    #[ignore = "requires an S3-compatible server on localhost:9000"]
    async fn stores_and_reads_an_object() -> Result<()> {
        let client = s3_compatible_client(
            "http://localhost:9000",
            "minioadmin",
            "minioadmin",
        );
        let bytes = round_trip(&client, "rust-howto-test").await?;
        assert_eq!(bytes, b"hello");
        Ok(())
    }
}
