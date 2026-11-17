//! A reqwest client checked against an in-process HTTP server.

#[cfg(test)]
use std::time::Duration;

use reqwest::header::ACCEPT;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[cfg_attr(
    not(test),
    expect(dead_code, reason = "used by the in-process router test")
)]
/// Data for greeting operations, including message.
struct Greeting {
    /// Message stored for this value.
    message: String,
}

// ANCHOR: example
#[cfg_attr(
    not(test),
    expect(dead_code, reason = "used by the in-process router test")
)]
async fn fetch_greeting(
    client: &reqwest::Client,
    base_url: &str,
    route: &str,
) -> Result<Greeting, reqwest::Error> {
    client
        .get(format!("{base_url}/{route}"))
        .header(ACCEPT, "application/json")
        .query(&[("name", "Ada")])
        .send()
        .await?
        .error_for_status()?
        .json::<Greeting>()
        .await
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use axum::Json;
    use axum::Router;
    use axum::extract::Query;
    use axum::http::HeaderMap;
    use axum::http::StatusCode;
    use axum::routing::get;

    use super::*;

    /// Handles the ok behavior for this component.
    async fn ok(
        headers: HeaderMap,
        Query(query): Query<std::collections::HashMap<String, String>>,
    ) -> Json<Greeting> {
        assert_eq!(
            headers.get(ACCEPT).and_then(|v| v.to_str().ok()),
            Some("application/json")
        );
        assert_eq!(query.get("name").map(String::as_str), Some("Ada"));
        Json(Greeting {
            message: "Hello, Ada!".into(),
        })
    }

    /// Checks that success status and bad json.
    #[tokio::test]
    async fn success_status_and_bad_json() -> anyhow::Result<()> {
        let app = Router::new()
            .route("/ok", get(ok))
            .route("/fail", get(|| async { StatusCode::SERVICE_UNAVAILABLE }))
            .route(
                "/bad",
                get(|| async { ([("content-type", "application/json")], "{bad") }),
            );
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
        let base_url = format!("http://{}", listener.local_addr()?);
        let server = tokio::spawn(async move { axum::serve(listener, app).await });
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(2))
            .build()?;
        assert_eq!(
            fetch_greeting(&client, &base_url, "ok").await?,
            Greeting {
                message: "Hello, Ada!".into()
            }
        );
        let status = fetch_greeting(&client, &base_url, "fail")
            .await
            .expect_err("503 is an error");
        assert_eq!(
            status.status(),
            Some(reqwest::StatusCode::SERVICE_UNAVAILABLE)
        );
        let malformed = fetch_greeting(&client, &base_url, "bad")
            .await
            .expect_err("malformed JSON is an error");
        assert!(malformed.is_decode());
        server.abort();
        Ok(())
    }
}

/// Runs the example demonstrated by this module.
fn main() {}
