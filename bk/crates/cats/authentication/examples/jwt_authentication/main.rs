//! Signs and verifies a JWT and demonstrates an Axum bearer-token extractor.

mod jwt;
mod jwt_axum_extractor;

use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use anyhow::Context as _;

/// Signs and verifies an in-memory token using the `JWT_SECRET` environment
/// variable.
fn main() -> anyhow::Result<()> {
    let secret = std::env::var("JWT_SECRET")
        .context("set JWT_SECRET to at least 32 random bytes")?;
    let jwt_secret = jwt_axum_extractor::JwtSecret::new(secret.as_bytes())?;
    let exp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context("system clock is before the Unix epoch")?
        .as_secs()
        + 60;
    let claims = jwt::Claims {
        sub: "example-user".to_owned(),
        exp,
        roles: vec!["reader".to_owned()],
    };
    let token = jwt::create_jwt(&claims, secret.as_bytes())?;
    let verified = jwt::verify_jwt(&token, secret.as_bytes())?;
    let _admin_router = jwt_axum_extractor::admin_router(jwt_secret);

    println!(
        "Created and verified a short-lived token for {}.",
        verified.sub
    );
    Ok(())
}
