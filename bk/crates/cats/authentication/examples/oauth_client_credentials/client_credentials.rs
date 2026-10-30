// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Obtains an OAuth 2.0 client-credentials token and calls a local mock API.
//!
//! The service requests a scoped token, then uses it to access a protected
//! resource. Local mock servers exercise both requests without an external
//! provider. The demo secret is a fixture; deployed services should load
//! secrets securely, request least-privilege scopes, and honor the returned
//! token expiry. This flow authorizes a service, not a human user; use OpenID
//! Connect for sign-in.

use std::time::Duration;

use anyhow::Context as _;
use httpmock::Method;
use httpmock::MockServer;
use oauth2::AuthUrl;
use oauth2::ClientId;
use oauth2::ClientSecret;
use oauth2::Scope;
use oauth2::TokenResponse as _;
use oauth2::TokenUrl;
use oauth2::basic::BasicClient;

/// Requests a scoped access token and uses it to call a protected resource.
///
/// # Errors
/// Returns an error if either server rejects the request or responds
/// unsuccessfully.
fn request_protected_resource(
    token_url: &str,
    resource_url: &str,
) -> anyhow::Result<(String, Option<Duration>)> {
    let oauth_client =
        BasicClient::new(ClientId::new("demo-client".to_owned()))
            .set_client_secret(ClientSecret::new("demo-secret".to_owned()))
            .set_auth_uri(AuthUrl::new(
                "http://auth.example.test/authorize".to_owned(),
            )?)
            .set_token_uri(TokenUrl::new(token_url.to_owned())?);

    let oauth_http_client = oauth2::reqwest::blocking::ClientBuilder::new()
        // Never follow redirects from a token endpoint.
        .redirect(oauth2::reqwest::redirect::Policy::none())
        .build()
        .context("build the OAuth token HTTP client")?;
    let token = oauth_client
        .exchange_client_credentials()
        .add_scope(Scope::new("records:read".to_owned()))
        .request(&oauth_http_client)
        .context("request a client-credentials access token")?;

    let expires_in = token.expires_in();
    let response = reqwest::blocking::Client::new()
        .get(resource_url)
        .bearer_auth(token.access_token().secret())
        .send()
        .context("send the bearer-authenticated resource request")?
        .error_for_status()
        .context("resource server returned an unsuccessful status")?;

    Ok((response.text()?, expires_in))
}

/// Runs the client against local authorization and resource server mocks.
///
/// # Errors
/// Returns an error if the token or resource request fails.
fn main() -> anyhow::Result<()> {
    let auth_server = MockServer::start();
    let resource_server = MockServer::start();

    let token_mock = auth_server.mock(|when, then| {
        when.method(Method::POST)
            .path("/token")
            .header(
                "authorization",
                "Basic ZGVtby1jbGllbnQ6ZGVtby1zZWNyZXQ=",
            )
            .body_includes("grant_type=client_credentials")
            .body_includes("scope=records%3Aread");
        then.status(200)
            .header("content-type", "application/json")
            .body(
                r#"{"access_token":"demo-access-token","token_type":"Bearer","expires_in":3600,"scope":"records:read"}"#,
            );
    });
    let resource_mock = resource_server.mock(|when, then| {
        when.method(Method::GET)
            .path("/records")
            .header("authorization", "Bearer demo-access-token");
        then.status(200).body("mock records");
    });

    let (body, expires_in) = request_protected_resource(
        &auth_server.url("/token"),
        &resource_server.url("/records"),
    )?;

    token_mock.assert();
    resource_mock.assert();
    assert_eq!(body, "mock records");
    assert_eq!(expires_in, Some(Duration::from_secs(3_600)));
    println!("Received the protected response: {body}");
    Ok(())
}
// ANCHOR_END: example

/// Runs the client-credentials example with its local mock servers.
///
/// # Errors
///
/// Returns an error if the mocked token or resource request fails.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn client_credentials_flow_uses_local_servers() -> anyhow::Result<()> {
        main()
    }

    #[test]
    fn token_endpoint_error_is_reported() {
        let auth_server = MockServer::start();
        let token_mock = auth_server.mock(|when, then| {
            when.method(Method::POST).path("/token");
            then.status(400)
                .header("content-type", "application/json")
                .body(r#"{"error":"invalid_client"}"#);
        });

        let err = request_protected_resource(
            &auth_server.url("/token"),
            "http://127.0.0.1:1/unused",
        )
        .expect_err("the token request is rejected");

        token_mock.assert();
        assert!(
            format!("{err:#}").contains("client-credentials access token"),
            "{err:#}"
        );
    }

    #[test]
    fn unauthorized_resource_response_is_an_error() {
        let auth_server = MockServer::start();
        let resource_server = MockServer::start();
        auth_server.mock(|when, then| {
            when.method(Method::POST).path("/token");
            then.status(200)
                .header("content-type", "application/json")
                .body(r#"{"access_token":"t","token_type":"Bearer"}"#);
        });
        let resource_mock = resource_server.mock(|when, then| {
            when.method(Method::GET).path("/records");
            then.status(401);
        });

        let err = request_protected_resource(
            &auth_server.url("/token"),
            &resource_server.url("/records"),
        )
        .expect_err("the resource server answers 401");

        resource_mock.assert();
        assert!(
            format!("{err:#}").contains("unsuccessful status"),
            "{err:#}"
        );
    }

    #[test]
    fn missing_expiry_is_reported_as_none() -> anyhow::Result<()> {
        let auth_server = MockServer::start();
        let resource_server = MockServer::start();
        auth_server.mock(|when, then| {
            when.method(Method::POST).path("/token");
            then.status(200)
                .header("content-type", "application/json")
                .body(r#"{"access_token":"t","token_type":"Bearer"}"#);
        });
        resource_server.mock(|when, then| {
            when.method(Method::GET).path("/records");
            then.status(200).body("ok");
        });

        let (body, expires_in) = request_protected_resource(
            &auth_server.url("/token"),
            &resource_server.url("/records"),
        )?;
        assert_eq!(body, "ok");
        assert_eq!(expires_in, None);
        Ok(())
    }
}
