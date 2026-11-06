// TODO Reviewed ??-??-2026.

#![cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "needs AWS credentials (CloudWatch); the tests use mocks"
    )
)]
// ANCHOR: example
//! Publish a custom metric with dimensions to Amazon CloudWatch.
//!
//! Amazon CloudWatch collects metrics and logs from AWS services. A custom
//! metric is a number your own program publishes, such as orders processed;
//! dimensions are labels, such as the environment, that let you slice it.
//! Running this needs AWS credentials.

use anyhow::Context;
use anyhow::Result;
use aws_sdk_cloudwatch::Client;
use aws_sdk_cloudwatch::types::Dimension;
use aws_sdk_cloudwatch::types::MetricDatum;
use aws_sdk_cloudwatch::types::StandardUnit;

/// Builds one data point. Each distinct set of dimensions is a separate metric
/// (and is billed as one), so keep dimension values low-cardinality.
pub fn latency_datum(service: &str, millis: f64) -> Result<MetricDatum> {
    let dimension = Dimension::builder().name("Service").value(service).build();
    Ok(MetricDatum::builder()
        .metric_name("RequestLatency")
        .dimensions(dimension)
        .unit(StandardUnit::Milliseconds)
        .value(millis)
        .build())
}

/// Sends the data points under `namespace`.
pub async fn put_metrics(client: &Client, namespace: &str, data: Vec<MetricDatum>) -> Result<()> {
    client
        .put_metric_data()
        .namespace(namespace)
        .set_metric_data(Some(data))
        .send()
        .await
        .context("CloudWatch PutMetricData failed")?;
    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies the metric name, value, unit and dimension.
    #[test]
    fn datum_carries_dimensions() -> Result<()> {
        let datum = latency_datum("api", 12.5)?;
        assert_eq!(datum.metric_name(), Some("RequestLatency"));
        assert_eq!(datum.value(), Some(12.5));
        assert_eq!(datum.unit(), Some(&StandardUnit::Milliseconds));
        assert_eq!(datum.dimensions().len(), 1);
        assert_eq!(datum.dimensions()[0].value(), Some("api"));
        Ok(())
    }

    /// Builds a client that targets LocalStack (`AWS_ENDPOINT_URL`, default
    /// `http://localhost:4566`) with dummy credentials.
    fn localstack_client() -> Client {
        let endpoint = std::env::var("AWS_ENDPOINT_URL")
            .unwrap_or_else(|_| "http://localhost:4566".to_owned());
        let config = aws_sdk_cloudwatch::Config::builder()
            .behavior_version(aws_sdk_cloudwatch::config::BehaviorVersion::latest())
            .region(aws_sdk_cloudwatch::config::Region::new("us-east-1"))
            .endpoint_url(endpoint)
            .credentials_provider(aws_sdk_cloudwatch::config::Credentials::new(
                "test", "test", None, None, "static",
            ))
            .build();
        Client::from_conf(config)
    }

    /// Sends a metric to LocalStack and lists it back. Start LocalStack first:
    /// `docker run -p 4566:4566 localstack/localstack`.
    #[tokio::test]
    #[ignore = "requires LocalStack (CloudWatch) on localhost:4566"]
    async fn metrics_are_stored() -> Result<()> {
        let client = localstack_client();
        put_metrics(&client, "RustHowto/Test", vec![latency_datum("api", 12.5)?]).await?;
        let listed = client
            .list_metrics()
            .namespace("RustHowto/Test")
            .send()
            .await?;
        assert!(
            listed
                .metrics()
                .iter()
                .any(|m| m.metric_name() == Some("RequestLatency"))
        );
        Ok(())
    }
}
