#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! This module provides examples of interacting with Elasticsearch using the
//! official Rust client.
//!
//! Set `ELASTIC_URL` to an HTTPS endpoint for a remote cluster. The default
//! `http://localhost:9200` is allowed only for local development. TLS
//! certificate and hostname validation remain enabled. Set both
//! `ELASTIC_USERNAME` and `ELASTIC_PASSWORD` to use basic authentication.

use anyhow::Context as _;
use anyhow::ensure;
use elasticsearch::BulkParts;
use elasticsearch::Elasticsearch;
use elasticsearch::Error;
use elasticsearch::IndexParts;
use elasticsearch::SearchParts;
use elasticsearch::auth::Credentials;
use elasticsearch::cat::CatIndicesParts;
use elasticsearch::http::request::JsonBody;
use elasticsearch::http::response::Response;
use elasticsearch::http::transport::SingleNodeConnectionPool;
use elasticsearch::http::transport::TransportBuilder;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use serde_json::json;
use url::Host;
use url::Url;

/// Represents a document that can be indexed in Elasticsearch.
#[derive(Debug, Serialize, Deserialize)]
struct MyDocument {
    /// The identifier of the document.
    id: i32,
    /// The title of the document.
    title: String,
}

/// Indexes a document in Elasticsearch, then searches for it.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create an asynchronous Elasticsearch client. Remote connections must
    // use HTTPS; only local development may use plain HTTP.
    let url = std::env::var("ELASTIC_URL")
        .unwrap_or_else(|_| "http://localhost:9200".into());
    let client = build_client(&url)?;

    let response: Response = index_document(&client).await?;
    println!("Index response: {response:?}");

    let search_result: serde_json::Value = search_document(&client).await?;
    println!("Search result: {search_result:?}");

    let bulk_response: bool = bulk_documents(&client).await?;
    println!("Bulk response: {bulk_response:?}");

    cat_indices(client).await?;

    Ok(())
}

/// Builds a client after validating its URL and optional basic credentials.
fn build_client(url: &str) -> anyhow::Result<Elasticsearch> {
    let endpoint = parse_endpoint(url)?;
    let credentials = credentials_for_endpoint(
        &endpoint,
        std::env::var("ELASTIC_USERNAME").ok(),
        std::env::var("ELASTIC_PASSWORD").ok(),
    )?;
    let pool = SingleNodeConnectionPool::new(endpoint);
    let builder = TransportBuilder::new(pool);
    let builder = match credentials {
        Some(credentials) => builder.auth(credentials),
        None => builder,
    };

    Ok(Elasticsearch::new(builder.build()?))
}

/// Parses the configured endpoint and restricts plaintext HTTP to loopback.
fn parse_endpoint(url: &str) -> anyhow::Result<Url> {
    let endpoint =
        Url::parse(url).context("ELASTIC_URL must be a valid URL")?;
    let local_http = endpoint.scheme() == "http"
        && endpoint.host().is_some_and(is_loopback_host);
    ensure!(
        endpoint.scheme() == "https" || local_http,
        "ELASTIC_URL must use HTTPS unless it targets localhost"
    );
    ensure!(
        endpoint.username().is_empty() && endpoint.password().is_none(),
        "put credentials in ELASTIC_USERNAME and ELASTIC_PASSWORD, not ELASTIC_URL"
    );

    Ok(endpoint)
}

/// Returns whether `host` names localhost or a loopback IP address.
fn is_loopback_host(host: Host<&str>) -> bool {
    match host {
        Host::Domain(domain) => domain.eq_ignore_ascii_case("localhost"),
        Host::Ipv4(address) => address.is_loopback(),
        Host::Ipv6(address) => address.is_loopback(),
    }
}

/// Builds optional basic credentials and requires TLS whenever they are set.
fn credentials_for_endpoint(
    endpoint: &Url,
    username: Option<String>,
    password: Option<String>,
) -> anyhow::Result<Option<Credentials>> {
    match (username, password) {
        (Some(username), Some(password)) => {
            ensure!(
                endpoint.scheme() == "https",
                "basic credentials require an HTTPS endpoint"
            );
            ensure!(
                !username.is_empty() && !password.is_empty(),
                "ELASTIC_USERNAME and ELASTIC_PASSWORD must not be empty"
            );
            Ok(Some(Credentials::Basic(username, password)))
        }
        (None, None) => Ok(None),
        _ => anyhow::bail!(
            "set both ELASTIC_USERNAME and ELASTIC_PASSWORD for basic authentication"
        ),
    }
}

/// Indexes a single document into Elasticsearch.
async fn index_document(client: &Elasticsearch) -> Result<Response, Error> {
    // Define a document to index:
    let doc = MyDocument {
        id: 1,
        title: "Rust with Elasticsearch".to_owned(),
    };
    // Index the document:
    let response = client
        .index(IndexParts::IndexId("my_index", "1"))
        .body(json!(doc))
        .send()
        .await?;
    Ok(response)
}

/// Searches for documents in Elasticsearch based on a query.
async fn search_document(
    client: &Elasticsearch,
) -> Result<serde_json::Value, Error> {
    // Search for the document:
    let search_response = client
        .search(SearchParts::Index(&["my_index"]))
        .from(0)
        .size(10)
        .body(json!({
            "query": {
                "match": {
                    "title": "Rust"
                }
            }
        }))
        .send()
        .await?;
    let search_result = search_response.json::<serde_json::Value>().await?;
    Ok(search_result)
}

/// Performs bulk indexing of multiple documents into Elasticsearch.
async fn bulk_documents(client: &Elasticsearch) -> anyhow::Result<bool> {
    let mut body: Vec<JsonBody<_>> = Vec::with_capacity(4);

    // Add the first operation and document:
    body.push(json!({"index": {"_id": "1"}}).into());
    body.push(
        json!({
            "id": 1,
            "user": "user1",
            "post_date": "2025-02-06T00:00:00Z",
            "message": "Trying out Elasticsearch"
        })
        .into(),
    );

    // Add the second operation and document:
    body.push(json!({"index": {"_id": "2"}}).into());
    body.push(
        json!({
            "id": 2,
            "user": "user2",
            "post_date": "2025-02-07T00:00:00Z",
            "message": "Bulk indexing with the rust client!"
        })
        .into(),
    );

    let response = client
        .bulk(BulkParts::Index("tweets"))
        .body(body)
        .send()
        .await?;

    let response_body = response.json::<Value>().await?;
    let successful =
        !(response_body["errors"].as_bool().ok_or(anyhow::anyhow!(
            "bulk_documents: response could not be converted to bool"
        ))?);

    Ok(successful)
}

/// Retrieves and displays information about indices in Elasticsearch.
async fn cat_indices(client: Elasticsearch) -> Result<Response, Error> {
    // Call the `Cat` related APIs.
    let response = client
        .cat()
        .indices(CatIndicesParts::Index(&["*"]))
        .format("json")
        .send()
        .await?;
    Ok(response)
}
// ANCHOR_END: example

pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Requires HTTPS for remote Elasticsearch endpoints.
    #[test]
    fn remote_endpoints_require_https() {
        assert!(parse_endpoint("http://search.example.com:9200").is_err());
        assert!(parse_endpoint("https://search.example.com:9200").is_ok());
    }

    /// Allows local HTTP endpoints but rejects non-loopback plaintext hosts.
    #[test]
    fn plaintext_is_limited_to_loopback_hosts() {
        assert!(parse_endpoint("http://localhost:9200").is_ok());
        assert!(parse_endpoint("http://127.0.0.1:9200").is_ok());
        assert!(parse_endpoint("http://[::1]:9200").is_ok());
        assert!(parse_endpoint("http://elasticsearch:9200").is_err());
    }

    /// Rejects credentials in URLs and incomplete basic-auth environment pairs.
    #[test]
    fn credentials_require_https_and_both_environment_values()
    -> anyhow::Result<()> {
        let https = parse_endpoint("https://search.example.com:9200")?;
        let http = parse_endpoint("http://localhost:9200")?;
        assert!(
            parse_endpoint("https://user:secret@search.example.com:9200")
                .is_err()
        );
        assert!(
            credentials_for_endpoint(&https, Some("user".into()), None)
                .is_err()
        );
        assert!(
            credentials_for_endpoint(
                &http,
                Some("user".into()),
                Some("secret".into()),
            )
            .is_err()
        );
        assert!(
            credentials_for_endpoint(
                &https,
                Some("user".into()),
                Some("secret".into()),
            )
            .is_ok()
        );
        Ok(())
    }

    /// Keeps the external service test out of ordinary workspace test runs.
    #[test]
    #[ignore = "Requires a disposable Elasticsearch service configured through ELASTIC_URL"]
    fn runs_against_configured_external_service() -> anyhow::Result<()> {
        main()
    }
}
// [review fix heavy test; secure the connection](https://github.com/john-cd/rust_howto/issues/710)
