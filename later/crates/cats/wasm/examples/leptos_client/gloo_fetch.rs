#![cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "targets the browser (wasm32); native builds cannot call it"
    )
)]
// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Call an HTTP API from WebAssembly with [`gloo_net`].
//!
//! In the browser, programs cannot use the usual networking libraries; they
//! must call `fetch`. `gloo_net` wraps it in a Rust async API. The server must
//! allow the request with CORS headers if it is on a different origin (domain).

/// Describes a GET request independently of the platform, so it can be tested
/// natively.
#[derive(Debug, PartialEq, Eq)]
pub struct RequestSpec {
    /// Fully qualified endpoint URL after joining the base and request path.
    pub url: String,
    /// Optional bearer token formatted for the `Authorization` header.
    pub authorization: Option<String>,
}

/// Builds the request for `path`, attaching a bearer token when supplied.
///
/// # Arguments
/// * `base` - API base URL, with or without a trailing slash.
/// * `path` - Relative endpoint path, with or without a leading slash.
/// * `token` - Optional access token used for bearer authorization.
pub fn build_request(base: &str, path: &str, token: Option<&str>) -> RequestSpec {
    RequestSpec {
        url: format!(
            "{}/{}",
            base.trim_end_matches('/'),
            path.trim_start_matches('/')
        ),
        authorization: token.map(|t| format!("Bearer {t}")),
    }
}

/// Sends the request and decodes the JSON body (browser only).
#[cfg(target_arch = "wasm32")]
pub async fn fetch_json<T: serde::de::DeserializeOwned>(
    spec: &RequestSpec,
) -> Result<T, gloo_net::Error> {
    use gloo_net::http::Request;

    let mut request = Request::get(&spec.url);
    if let Some(auth) = &spec.authorization {
        request = request.header("Authorization", auth);
    }
    request.send().await?.json::<T>().await
}
// ANCHOR_END: example

/// Builds a representative API request.
fn run() -> RequestSpec {
    build_request("https://api.example.com", "/items", None)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Joins the request URL and appends its authentication token.
    #[test]
    fn joins_url_and_adds_token() {
        let sample = run();
        assert_eq!(sample.url, "https://api.example.com/items");
        let spec = build_request("https://api.example.com/", "/items", Some("abc"));
        assert_eq!(spec.url, "https://api.example.com/items");
        assert_eq!(spec.authorization.as_deref(), Some("Bearer abc"));
        assert_eq!(build_request("http://x", "y", None).authorization, None);
    }
}
