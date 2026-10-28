// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Extract a JWT from the HTTP `Authorization` header and check a role in Axum.

use std::sync::Arc;

use axum::Extension;
use axum::Router;
use axum::extract::FromRequestParts;
use axum::http::StatusCode;
use axum::http::header::AUTHORIZATION;
use axum::http::request::Parts;
use axum::routing::get;

use super::jwt;
use super::jwt::Claims;

/// Holds the HS256 key shared by the router's request extractors.
#[derive(Clone)]
pub struct JwtSecret(Arc<[u8]>);

impl JwtSecret {
    /// Creates a key after checking the minimum HS256 key size.
    ///
    /// # Errors
    /// Returns an error when `secret` is shorter than 32 bytes.
    pub fn new(secret: impl AsRef<[u8]>) -> anyhow::Result<Self> {
        let secret = secret.as_ref();
        anyhow::ensure!(
            secret.len() >= 32,
            "an HS256 secret must contain at least 32 bytes"
        );
        Ok(Self(Arc::from(secret)))
    }

    /// Returns the key bytes for signature verification inside this module.
    fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

/// Claims extracted from a valid bearer token.
pub struct AuthClaims(Claims);

impl<S> FromRequestParts<S> for AuthClaims
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    /// Reads and verifies a bearer token without consuming the request body.
    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let Extension(secret) =
            Extension::<JwtSecret>::from_request_parts(parts, state)
                .await
                .map_err(|_| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "JWT validation key is not configured",
                    )
                })?;
        let token = bearer_token(parts)
            .ok_or((StatusCode::UNAUTHORIZED, "a bearer token is required"))?;
        let claims =
            jwt::verify_jwt(token, secret.as_bytes()).map_err(|_| {
                (StatusCode::UNAUTHORIZED, "the bearer token is invalid")
            })?;

        Ok(Self(claims))
    }
}

/// Returns the token only when the `Authorization` header uses Bearer syntax.
fn bearer_token(parts: &Parts) -> Option<&str> {
    let value = parts.headers.get(AUTHORIZATION)?.to_str().ok()?;
    let mut words = value.split_ascii_whitespace();
    let scheme = words.next()?;
    let token = words.next()?;
    (scheme.eq_ignore_ascii_case("Bearer") && words.next().is_none())
        .then_some(token)
}

/// Returns whether `claims` contains the requested application role.
pub fn role_allowed(claims: &Claims, required_role: &str) -> bool {
    claims.roles.iter().any(|role| role == required_role)
}

/// Builds a small router whose admin endpoint requires a valid bearer token and
/// role.
pub fn admin_router(secret: JwtSecret) -> Router {
    Router::new()
        .route("/admin", get(admin_route))
        .layer(Extension(secret))
}

/// Returns 200 for an admin token and 403 for a valid token without that role.
async fn admin_route(AuthClaims(claims): AuthClaims) -> StatusCode {
    if role_allowed(&claims, "admin") {
        StatusCode::OK
    } else {
        StatusCode::FORBIDDEN
    }
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::HeaderValue;
    use axum::http::Request;
    use tower::ServiceExt;

    use super::*;

    const TEST_SECRET: &[u8] = b"unit-test-secret-with-at-least-32-bytes";

    /// Signs a valid token for the requested roles using the test key.
    fn token_for_roles(roles: &[&str]) -> String {
        let claims = Claims {
            sub: "alice".to_owned(),
            exp: u64::MAX,
            roles: roles.iter().map(|role| (*role).to_owned()).collect(),
        };
        jwt::create_jwt(&claims, TEST_SECRET)
            .expect("the test key should sign the test claims")
    }

    /// Builds a request with an optional bearer token.
    fn request_with_token(token: Option<&str>) -> Request<Body> {
        let mut request = Request::new(Body::empty());
        *request.uri_mut() = axum::http::Uri::from_static("/admin");
        if let Some(token) = token {
            let header = HeaderValue::from_str(&format!("Bearer {token}"))
                .expect("a JWT token contains valid header characters");
            request.headers_mut().insert(AUTHORIZATION, header);
        }
        request
    }

    /// Sends a request through the router and returns its status code.
    async fn call_admin(token: Option<&str>) -> StatusCode {
        let secret =
            JwtSecret::new(TEST_SECRET).expect("the test key is long enough");
        admin_router(secret)
            .oneshot(request_with_token(token))
            .await
            .expect("the example router has an infallible service error")
            .status()
    }

    /// Rejects a request that has no bearer token.
    #[tokio::test]
    async fn missing_bearer_token_returns_401() {
        assert_eq!(call_admin(None).await, StatusCode::UNAUTHORIZED);
    }

    /// Rejects a bearer token with an invalid signature.
    #[tokio::test]
    async fn invalid_bearer_token_returns_401() {
        assert_eq!(
            call_admin(Some("not.a.valid-token")).await,
            StatusCode::UNAUTHORIZED
        );
    }

    /// Rejects a valid token when its subject lacks the admin role.
    #[tokio::test]
    async fn missing_admin_role_returns_403() {
        let token = token_for_roles(&["reader"]);

        assert_eq!(call_admin(Some(&token)).await, StatusCode::FORBIDDEN);
    }

    /// Allows a valid token when its subject has the admin role.
    #[tokio::test]
    async fn admin_role_returns_200() {
        let token = token_for_roles(&["admin"]);

        assert_eq!(call_admin(Some(&token)).await, StatusCode::OK);
    }
}
