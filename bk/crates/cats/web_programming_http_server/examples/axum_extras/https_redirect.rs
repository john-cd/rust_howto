// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Redirect HTTP requests to HTTPS.
//!
//! The URI rewriting is a pure function, which makes it easy to test.

use axum::Router;
use axum::http::Uri;
use axum::http::uri::Authority;
use axum::http::uri::Scheme;
use axum::response::Redirect;
use axum::routing::get;
use axum_extra::TypedHeader;
use axum_extra::headers::Host as HostHeader;

/// Ports of the plain HTTP and HTTPS listeners.
#[derive(Clone, Copy, Debug)]
pub struct Ports {
    /// HTTP port used by the local endpoint.
    pub http: u16,
    /// HTTPS port used by the local endpoint.
    pub https: u16,
}

/// Rewrites `uri` to use `https`, replacing the HTTP port in `host`.
///
/// # Arguments
/// * `uri` - URI identifying the requested resource.
pub fn make_https(host: &str, uri: &Uri, ports: Ports) -> anyhow::Result<Uri> {
    let mut parts = uri.clone().into_parts();
    parts.scheme = Some(Scheme::HTTPS);
    if parts.path_and_query.is_none() {
        parts.path_and_query = Some("/".parse()?);
    }

    // Drop the HTTP port, if any, and add the HTTPS one.
    let name = host
        .strip_suffix(&format!(":{}", ports.http))
        .unwrap_or(host);
    let authority = if ports.https == 443 {
        name.to_owned()
    } else {
        format!("{name}:{}", ports.https)
    };
    parts.authority = Some(authority.parse::<Authority>()?);
    Ok(Uri::from_parts(parts)?)
}

/// Builds a router that redirects every request to HTTPS.
pub fn app(ports: Ports) -> Router {
    Router::new().fallback(get(
        move |TypedHeader(host): TypedHeader<HostHeader>, uri: Uri| async move {
            match make_https(&host.to_string(), &uri, ports) {
                Ok(uri) => Ok(Redirect::permanent(&uri.to_string())),
                Err(_) => Err(axum::http::StatusCode::BAD_REQUEST),
            }
        },
    ))
}
// ANCHOR_END: example

/// Builds the router that redirects HTTP requests to HTTPS.
pub fn run() -> Router {
    app(Ports {
        http: 8080,
        https: 8443,
    })
}

/// An error is returned if replaces_port_and_scheme, make_https fails.
#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::Request;
    use axum::http::StatusCode;
    use tower::ServiceExt as _;

    use super::*;

    /// Shared `ports` used by this module.
    const PORTS: Ports = Ports {
        http: 8080,
        https: 8443,
    };

    /// # Errors
    /// An error is returned if replaces_port_and_scheme, make_https fails.
    #[test]
    fn replaces_port_and_scheme() -> anyhow::Result<()> {
        let uri: Uri = "/a/b?c=d".parse()?;
        let out = make_https("example.com:8080", &uri, PORTS)?;
        assert_eq!(out.to_string(), "https://example.com:8443/a/b?c=d");
        Ok(())
    }

    /// # Errors
    /// An error is returned if make_https fails.
    #[test]
    fn default_https_port_is_omitted() -> anyhow::Result<()> {
        let ports = Ports {
            http: 80,
            https: 443,
        };
        let out = make_https("example.com", &"/".parse()?, ports)?;
        assert_eq!(out.to_string(), "https://example.com/");
        Ok(())
    }

    /// # Errors
    /// An error is returned if invalid_host_is_an_error fails.
    #[test]
    fn invalid_host_is_an_error() -> anyhow::Result<()> {
        assert!(make_https("bad host", &"/".parse()?, PORTS).is_err());
        Ok(())
    }

    /// # Errors
    /// An error is returned if redirects fails.
    #[tokio::test]
    async fn redirects() -> anyhow::Result<()> {
        let request = Request::get("/login?next=%2F")
            .header("host", "example.com:8080")
            .body(Body::empty())?;
        let response = app(PORTS).oneshot(request).await?;
        assert_eq!(response.status(), StatusCode::PERMANENT_REDIRECT);
        assert_eq!(
            response.headers()["location"],
            "https://example.com:8443/login?next=%2F"
        );
        Ok(())
    }
}
