// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Signs a user in with the OpenID Connect authorization-code flow and PKCE,
//! using a deterministic in-process identity provider.
//!
//! The relying party (this program):
//!
//! 1. builds an authorization URL carrying `state`, `nonce` and a PKCE
//!    challenge, and remembers the matching secrets for this one login;
//! 2. on the callback, rejects a `state` it did not issue;
//! 3. exchanges the code, proving possession of the PKCE verifier;
//! 4. verifies the **ID token**: signature, issuer, audience, expiry and nonce.
//!    The **access token** is opaque to the client: it is only presented to the
//!    provider's APIs and is never parsed or trusted here.
//!
//! The token endpoint is a closure returning a signed fixture, so no network
//! or browser is involved. The fixture signs with HS256 and the client secret
//! (allowed by OpenID Connect for confidential clients); real providers
//! usually sign with RS256 or ES256 and publish their keys at a JWKS URL,
//! which `CoreProviderMetadata::discover` fetches. Only discover from an
//! issuer URL you trust and configured yourself.

use anyhow::Context as _;
use anyhow::bail;
use jsonwebtoken::EncodingKey;
use jsonwebtoken::Header;
use openidconnect::AuthUrl;
use openidconnect::AuthorizationCode;
use openidconnect::ClientId;
use openidconnect::ClientSecret;
use openidconnect::CsrfToken;
use openidconnect::EmptyAdditionalProviderMetadata;
use openidconnect::EndpointMaybeSet;
use openidconnect::EndpointNotSet;
use openidconnect::EndpointSet;
use openidconnect::HttpRequest;
use openidconnect::HttpResponse;
use openidconnect::IssuerUrl;
use openidconnect::JsonWebKeySetUrl;
use openidconnect::Nonce;
use openidconnect::PkceCodeChallenge;
use openidconnect::PkceCodeVerifier;
use openidconnect::RedirectUrl;
use openidconnect::ResponseTypes;
use openidconnect::Scope;
use openidconnect::SyncHttpClient;
use openidconnect::TokenResponse as _;
use openidconnect::TokenUrl;
use openidconnect::core::CoreAuthenticationFlow;
use openidconnect::core::CoreClient;
use openidconnect::core::CoreJwsSigningAlgorithm;
use openidconnect::core::CoreProviderMetadata;
use openidconnect::core::CoreResponseType;
use openidconnect::core::CoreSubjectIdentifierType;
use openidconnect::url::Url;
use serde_json::json;

/// The provider's issuer identifier.
const ISSUER: &str = "https://idp.example.test";
/// The identifier registered for this application.
const CLIENT_ID: &str = "demo-client";
/// A demo secret; real secrets come from a secret store.
const CLIENT_SECRET: &str = "demo-secret-demo-secret-demo-secret";
/// Where the provider sends the browser back.
const REDIRECT: &str = "https://app.example.test/callback";

/// Secrets that must survive between the redirect and the callback, and be
/// used for that single login only.
struct PendingLogin {
    /// Binds the callback to this browser session.
    csrf: CsrfToken,
    /// Binds the ID token to this authorization request.
    nonce: Nonce,
    /// Proves the code is redeemed by the party that asked for it.
    pkce_verifier: PkceCodeVerifier,
}

/// What the application learns about the signed-in user.
#[derive(Debug, PartialEq, Eq)]
struct Identity {
    /// The provider-scoped, stable user identifier (`sub`).
    subject: String,
    /// The user's email, if the provider released it.
    email: Option<String>,
}

/// The client type produced by `from_provider_metadata` with a token endpoint.
type Client = CoreClient<
    EndpointSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointMaybeSet,
    EndpointMaybeSet,
>;

/// Builds the client from provider metadata that the application configured
/// itself: no discovery request is made.
fn build_client(issuer: &str) -> anyhow::Result<Client> {
    let metadata = CoreProviderMetadata::new(
        IssuerUrl::new(issuer.to_owned())?,
        AuthUrl::new(format!("{ISSUER}/authorize"))?,
        JsonWebKeySetUrl::new(format!("{ISSUER}/jwks"))?,
        vec![ResponseTypes::new(vec![CoreResponseType::Code])],
        vec![CoreSubjectIdentifierType::Public],
        vec![CoreJwsSigningAlgorithm::HmacSha256],
        EmptyAdditionalProviderMetadata {},
    )
    .set_token_endpoint(Some(TokenUrl::new(format!("{ISSUER}/token"))?));
    Ok(CoreClient::from_provider_metadata(
        metadata,
        ClientId::new(CLIENT_ID.to_owned()),
        Some(ClientSecret::new(CLIENT_SECRET.to_owned())),
    )
    .set_redirect_uri(RedirectUrl::new(REDIRECT.to_owned())?))
}

/// Step 1: returns the URL to send the browser to, and the secrets to keep.
fn start_login(client: &Client) -> (Url, PendingLogin) {
    let (challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
    let (url, csrf, nonce) = client
        .authorize_url(
            CoreAuthenticationFlow::AuthorizationCode,
            CsrfToken::new_random,
            Nonce::new_random,
        )
        .add_scope(Scope::new("email".to_owned()))
        .set_pkce_challenge(challenge)
        .url();
    (
        url,
        PendingLogin {
            csrf,
            nonce,
            pkce_verifier,
        },
    )
}

/// Steps 2 to 4: handles the callback and verifies the ID token.
///
/// `http` performs the token request; production code passes a client that
/// never follows redirects and has timeouts.
fn finish_login<C>(
    client: &Client,
    pending: PendingLogin,
    returned_state: &str,
    code: &str,
    http: &C,
) -> anyhow::Result<Identity>
where
    C: SyncHttpClient,
{
    // Reject a callback that this browser session did not start.
    if returned_state != pending.csrf.secret() {
        bail!("the `state` parameter does not match this login");
    }
    let tokens = client
        .exchange_code(AuthorizationCode::new(code.to_owned()))?
        .set_pkce_verifier(pending.pkce_verifier)
        .request(http)
        .map_err(|error| {
            anyhow::anyhow!("exchange the authorization code: {error}")
        })?;
    let id_token = tokens
        .id_token()
        .context("the provider returned no ID token")?;
    // Checks signature, issuer, audience, expiry and the nonce.
    let claims = id_token
        .claims(&client.id_token_verifier(), &pending.nonce)
        .context("verify the ID token")?;
    Ok(Identity {
        subject: claims.subject().to_string(),
        email: claims.email().map(|email| email.to_string()),
    })
}

/// Claims for a fixture ID token.
fn id_token_claims(
    issuer: &str,
    audience: &str,
    nonce: &str,
    expires_at: i64,
) -> serde_json::Value {
    json!({
        "iss": issuer,
        "sub": "user-1234",
        "aud": [audience],
        "exp": expires_at,
        "iat": expires_at - 3600,
        "nonce": nonce,
        "email": "ada@example.test",
    })
}

/// Signs `claims` the way the fixture provider does.
fn sign(claims: &serde_json::Value, secret: &str) -> anyhow::Result<String> {
    Ok(jsonwebtoken::encode(
        &Header::new(jsonwebtoken::Algorithm::HS256),
        claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?)
}

/// A token endpoint that checks the PKCE verifier and returns `id_token`.
fn token_endpoint(
    id_token: String,
    expected_verifier: String,
) -> impl Fn(HttpRequest) -> Result<HttpResponse, std::convert::Infallible> {
    move |request: HttpRequest| {
        let form: Vec<(String, String)> =
            openidconnect::url::form_urlencoded::parse(request.body())
                .into_owned()
                .collect();
        let verifier_ok = form.iter().any(|(key, value)| {
            key == "code_verifier" && *value == expected_verifier
        });
        let body = if verifier_ok {
            json!({
                "access_token": "opaque-access-token",
                "token_type": "Bearer",
                "expires_in": 3600,
                "id_token": id_token,
            })
        } else {
            json!({ "error": "invalid_grant" })
        };
        let status = if verifier_ok { 200 } else { 400 };
        Ok(openidconnect::http::Response::builder()
            .status(status)
            .header("content-type", "application/json")
            .body(body.to_string().into_bytes())
            .expect("a static response is valid"))
    }
}

/// Runs a complete login against the fixture provider.
fn main() -> anyhow::Result<()> {
    let client = build_client(ISSUER)?;
    let (url, pending) = start_login(&client);
    println!("Send the browser to: {url}");

    // The provider would authenticate the user, then redirect back with
    // `?code=...&state=...`. Simulate its reply.
    let state = pending.csrf.secret().clone();
    let expires_at = unix_now() + 3600;
    let id_token = sign(
        &id_token_claims(ISSUER, CLIENT_ID, pending.nonce.secret(), expires_at),
        CLIENT_SECRET,
    )?;
    let http = token_endpoint(id_token, pending.pkce_verifier.secret().clone());

    let identity = finish_login(&client, pending, &state, "demo-code", &http)?;
    println!("Signed in {identity:?}");
    Ok(())
}

/// The current Unix time in seconds.
fn unix_now() -> i64 {
    let elapsed = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("the clock is after 1970");
    i64::try_from(elapsed.as_secs()).expect("the time fits in i64")
}
// ANCHOR_END: example

/// Runs the example.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A login attempt prepared like `main` does, with the fixture pieces
    /// that tests tweak.
    struct Attempt {
        /// The client.
        client: Client,
        /// The pending login.
        pending: PendingLogin,
        /// The state the provider will return.
        state: String,
    }

    /// Starts a login.
    fn attempt() -> Attempt {
        let client = build_client(ISSUER).expect("the metadata is valid");
        let (_url, pending) = start_login(&client);
        let state = pending.csrf.secret().clone();
        Attempt {
            client,
            pending,
            state,
        }
    }

    /// Finishes `attempt` with an ID token built from `claims` and signed
    /// with `secret`.
    fn finish_with(
        attempt: Attempt,
        claims: &serde_json::Value,
        secret: &str,
    ) -> anyhow::Result<Identity> {
        let id_token = sign(claims, secret)?;
        let http = token_endpoint(
            id_token,
            attempt.pending.pkce_verifier.secret().clone(),
        );
        finish_login(
            &attempt.client,
            attempt.pending,
            &attempt.state,
            "demo-code",
            &http,
        )
    }

    /// Valid claims for `attempt`.
    fn good_claims(attempt: &Attempt) -> serde_json::Value {
        id_token_claims(
            ISSUER,
            CLIENT_ID,
            attempt.pending.nonce.secret(),
            unix_now() + 3600,
        )
    }

    /// The authorization URL carries state, nonce and an S256 challenge.
    #[test]
    fn authorization_url_has_state_nonce_and_pkce() {
        let client = build_client(ISSUER).expect("valid metadata");
        let (url, pending) = start_login(&client);
        let query: Vec<(String, String)> = url
            .query_pairs()
            .map(|(k, v)| (k.into_owned(), v.into_owned()))
            .collect();
        let get = |name: &str| {
            query
                .iter()
                .find(|(key, _)| key == name)
                .map(|(_, value)| value.as_str())
        };
        assert_eq!(get("response_type"), Some("code"));
        assert_eq!(get("state"), Some(pending.csrf.secret().as_str()));
        assert_eq!(get("nonce"), Some(pending.nonce.secret().as_str()));
        assert_eq!(get("code_challenge_method"), Some("S256"));
        assert!(get("code_challenge").is_some());
        assert!(get("scope").is_some_and(|s| s.contains("openid")));
    }

    /// The happy path yields the verified identity.
    #[test]
    fn valid_login_returns_the_identity() {
        let attempt = attempt();
        let claims = good_claims(&attempt);
        let identity = finish_with(attempt, &claims, CLIENT_SECRET)
            .expect("a valid ID token is accepted");
        assert_eq!(identity.subject, "user-1234");
        assert_eq!(identity.email.as_deref(), Some("ada@example.test"));
    }

    /// A callback with a foreign `state` is rejected before any request.
    #[test]
    fn state_mismatch_is_rejected() {
        let mut attempt = attempt();
        attempt.state = "attacker-state".to_owned();
        let claims = good_claims(&attempt);
        assert!(finish_with(attempt, &claims, CLIENT_SECRET).is_err());
    }

    /// An ID token minted for another authorization request is rejected.
    #[test]
    fn nonce_mismatch_is_rejected() {
        let attempt = attempt();
        let claims = id_token_claims(
            ISSUER,
            CLIENT_ID,
            "some-other-nonce",
            unix_now() + 3600,
        );
        assert!(finish_with(attempt, &claims, CLIENT_SECRET).is_err());
    }

    /// Tokens from another issuer are rejected.
    #[test]
    fn wrong_issuer_is_rejected() {
        let attempt = attempt();
        let claims = id_token_claims(
            "https://evil.example.test",
            CLIENT_ID,
            attempt.pending.nonce.secret(),
            unix_now() + 3600,
        );
        assert!(finish_with(attempt, &claims, CLIENT_SECRET).is_err());
    }

    /// Tokens issued to another client are rejected.
    #[test]
    fn wrong_audience_is_rejected() {
        let attempt = attempt();
        let claims = id_token_claims(
            ISSUER,
            "someone-else",
            attempt.pending.nonce.secret(),
            unix_now() + 3600,
        );
        assert!(finish_with(attempt, &claims, CLIENT_SECRET).is_err());
    }

    /// Expired tokens are rejected.
    #[test]
    fn expired_token_is_rejected() {
        let attempt = attempt();
        let claims = id_token_claims(
            ISSUER,
            CLIENT_ID,
            attempt.pending.nonce.secret(),
            unix_now() - 7200,
        );
        assert!(finish_with(attempt, &claims, CLIENT_SECRET).is_err());
    }

    /// Tokens with an invalid signature are rejected.
    #[test]
    fn invalid_signature_is_rejected() {
        let attempt = attempt();
        let claims = good_claims(&attempt);
        assert!(
            finish_with(attempt, &claims, "another-secret-another-secret-xx")
                .is_err()
        );
    }

    /// The provider refuses a code redeemed with the wrong PKCE verifier.
    #[test]
    fn wrong_pkce_verifier_is_rejected() {
        let attempt = attempt();
        let claims = good_claims(&attempt);
        let id_token = sign(&claims, CLIENT_SECRET).expect("signing works");
        let http = token_endpoint(id_token, "a-different-verifier".to_owned());
        assert!(
            finish_login(
                &attempt.client,
                attempt.pending,
                &attempt.state,
                "demo-code",
                &http,
            )
            .is_err()
        );
    }
}
