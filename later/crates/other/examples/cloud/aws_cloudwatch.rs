// TODO Reviewed ??-??-2026.

#![allow(
    dead_code,
    reason = "the snippet declares items to illustrate them, not to call them"
)]
// ANCHOR: example
//! Publish a custom metric with dimensions to Amazon CloudWatch.

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
pub async fn put_metrics(
    client: &Client,
    namespace: &str,
    data: Vec<MetricDatum>,
) -> Result<()> {
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
}
