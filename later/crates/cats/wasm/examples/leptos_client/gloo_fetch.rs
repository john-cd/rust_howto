#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Call an HTTP API from WebAssembly with [`gloo_net`].

/// Describes a GET request independently of the platform, so it can be tested
/// natively.
#[derive(Debug, PartialEq, Eq)]
pub struct RequestSpec {
    pub url: String,
    pub authorization: Option<String>,
}

/// Builds the request for `path`, with a bearer token when the user is
/// logged in.
pub fn build_request(
    base: &str,
    path: &str,
    token: Option<&str>,
) -> RequestSpec {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn joins_url_and_adds_token() {
        let spec =
            build_request("https://api.example.com/", "/items", Some("abc"));
        assert_eq!(spec.url, "https://api.example.com/items");
        assert_eq!(spec.authorization.as_deref(), Some("Bearer abc"));
        assert_eq!(build_request("http://x", "y", None).authorization, None);
    }
}
