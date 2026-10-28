// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Encode and verify HS256 JSON Web Tokens for a first-party API.
//!
//! This module provides functions to create and verify JSON Web Tokens (JWTs)
//! using the HS256 algorithm. It is used in the JWT authentication example to
//! demonstrate how to handle short-lived bearer tokens in a first-party API.

use jsonwebtoken::Algorithm;
use jsonwebtoken::DecodingKey;
use jsonwebtoken::EncodingKey;
use jsonwebtoken::Header;
use jsonwebtoken::Validation;
use jsonwebtoken::decode;
use jsonwebtoken::encode;
use serde::Deserialize;
use serde::Serialize;

// TODO explain the purpose of this module and how it fits into the overall
// example. TODO explain the field names
/// Claims carried by the example's short-lived bearer token.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Claims {
    /// The subject identifier for the authenticated user.
    pub sub: String,
    /// Expiration time as seconds since the Unix epoch.
    pub exp: u64,
    /// Application roles used by the Axum authorization example.
    #[serde(default)]
    pub roles: Vec<String>,
}

/// Encodes `claims` as an HS256 token signed with `secret`.
///
/// # Errors
/// Returns an error if the claims cannot be serialized or signed.
pub fn create_jwt(
    claims: &Claims,
    secret: &[u8],
) -> Result<String, jsonwebtoken::errors::Error> {
    encode(
        &Header::new(Algorithm::HS256),
        claims,
        &EncodingKey::from_secret(secret),
    )
}

/// Verifies the HS256 signature and expiration, then returns the token claims.
///
/// # Errors
/// Returns an error if the token is malformed, expired, or signed with another
/// key.
pub fn verify_jwt(
    token: &str,
    secret: &[u8],
) -> Result<Claims, jsonwebtoken::errors::Error> {
    let validation = Validation::new(Algorithm::HS256);
    decode::<Claims>(token, &DecodingKey::from_secret(secret), &validation)
        .map(|data| data.claims)
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    /// Returns deterministic claims that remain valid for signature tests.
    fn unexpired_claims() -> Claims {
        Claims {
            sub: "alice".to_owned(),
            exp: u64::MAX,
            roles: vec!["reader".to_owned()],
        }
    }

    /// Signs and verifies claims without a network service.
    #[test]
    fn valid_token_round_trips_claims()
    -> Result<(), jsonwebtoken::errors::Error> {
        let claims = unexpired_claims();
        let token =
            create_jwt(&claims, b"unit-test-secret-with-at-least-32-bytes")?;

        assert_eq!(
            verify_jwt(&token, b"unit-test-secret-with-at-least-32-bytes")?,
            claims
        );
        Ok(())
    }

    /// Rejects a correctly signed token whose expiration is in the past.
    #[test]
    fn expired_token_is_rejected() -> Result<(), jsonwebtoken::errors::Error> {
        let claims = Claims {
            exp: 1,
            ..unexpired_claims()
        };
        let token =
            create_jwt(&claims, b"unit-test-secret-with-at-least-32-bytes")?;

        assert!(
            verify_jwt(&token, b"unit-test-secret-with-at-least-32-bytes")
                .is_err()
        );
        Ok(())
    }

    /// Rejects a token after its signature segment has been changed.
    #[test]
    fn tampered_signature_is_rejected()
    -> Result<(), jsonwebtoken::errors::Error> {
        let token = create_jwt(
            &unexpired_claims(),
            b"unit-test-secret-with-at-least-32-bytes",
        )?;
        let (message, signature) = token
            .rsplit_once('.')
            .expect("a signed JWT has a signature segment");
        let mut tampered_signature = signature.as_bytes().to_vec();
        tampered_signature[0] = if tampered_signature[0] == b'A' {
            b'B'
        } else {
            b'A'
        };
        let tampered_signature = String::from_utf8(tampered_signature)
            .expect("JWT signatures use ASCII characters");
        let tampered_token = format!("{message}.{tampered_signature}");

        assert!(
            verify_jwt(
                &tampered_token,
                b"unit-test-secret-with-at-least-32-bytes"
            )
            .is_err()
        );
        Ok(())
    }
}
