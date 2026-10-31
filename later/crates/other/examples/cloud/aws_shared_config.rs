// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Load the AWS configuration once and share it across service clients.
//!
//! The SDK's configuration (credentials, region, retry policy) is loaded once,
//! which involves reading files and the environment, and then shared by every
//! service client, so the program does not repeat that work.

use std::time::Duration;

use aws_config::BehaviorVersion;
use aws_config::Region;
use aws_config::SdkConfig;
use aws_config::retry::RetryConfig;
use aws_config::timeout::TimeoutConfig;

/// Loads the configuration with the default credential provider chain
/// (environment, shared profile, SSO, container and instance roles).
///
/// `region` and `profile` override what the environment provides. Retries and
/// timeouts are set explicitly rather than left to the defaults.
pub async fn load_config(
    region: Option<&str>,
    profile: Option<&str>,
) -> SdkConfig {
    // Pin a behavior version so SDK upgrades do not change defaults silently.
    let mut loader = aws_config::defaults(BehaviorVersion::latest())
        .retry_config(RetryConfig::standard().with_max_attempts(5))
        .timeout_config(
            TimeoutConfig::builder()
                .operation_timeout(Duration::from_secs(30))
                .build(),
        );
    if let Some(region) = region {
        loader = loader.region(Region::new(region.to_owned()));
    }
    if let Some(profile) = profile {
        loader = loader.profile_name(profile);
    }
    loader.load().await
}

/// Builds several service clients from the same configuration.
pub async fn clients() -> (aws_sdk_s3::Client, aws_sdk_sqs::Client) {
    let sdk_config = load_config(Some("us-east-1"), None).await;
    // Each client reuses the loaded region, credentials and HTTP settings.
    (
        aws_sdk_s3::Client::new(&sdk_config),
        aws_sdk_sqs::Client::new(&sdk_config),
    )
}
// ANCHOR_END: example

/// Runs the example: builds the clients and prints their region.
pub fn run() -> anyhow::Result<()> {
    let (s3, sqs) = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?
        .block_on(clients());
    println!("S3 region: {:?}", s3.config().region());
    println!("SQS region: {:?}", sqs.config().region());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that explicit settings are applied without calling AWS.
    #[tokio::test]
    async fn overrides_are_applied() {
        let config = load_config(Some("eu-west-1"), None).await;
        assert_eq!(config.region().map(Region::as_ref), Some("eu-west-1"));
        let retry = config.retry_config().expect("retry config is set");
        assert_eq!(retry.max_attempts(), 5);
    }

    /// Verifies that clients created from one configuration share its region.
    #[tokio::test]
    async fn clients_share_the_region() {
        let (s3, sqs) = clients().await;
        assert_eq!(s3.config().region(), sqs.config().region());
    }
}
