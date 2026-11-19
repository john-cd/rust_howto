//! Verifies that an Axum session survives rebuilding the application.

// ANCHOR: example
use axum::Router;
use axum::body::Body;
use axum::http::Request;
use axum::http::header;
use axum::routing::get;
use axum::routing::post;
use http_body_util::BodyExt;
use tower::ServiceExt;
use tower_sessions::Session;
use tower_sessions::SessionManagerLayer;
use tower_sessions_sqlx_store::SqliteStore;
use tower_sessions_sqlx_store::sqlx::sqlite::SqliteConnectOptions;
use tower_sessions_sqlx_store::sqlx::sqlite::SqlitePool;

/// Saves a demo preference in the session.
async fn remember(session: Session) -> Result<(), axum::http::StatusCode> {
    session
        .insert("theme", "dark")
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)
}

/// Loads the preference identified by the session cookie.
async fn theme(session: Session) -> Result<String, axum::http::StatusCode> {
    session
        .get("theme")
        .await
        .map(|value: Option<String>| value.unwrap_or_else(|| "default".into()))
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)
}

/// Configures the session store for the provided database.
async fn app(pool: SqlitePool) -> anyhow::Result<Router> {
    let store = SqliteStore::new(pool);
    store.migrate().await?;
    Ok(Router::new()
        .route("/remember", post(remember))
        .route("/theme", get(theme))
        .layer(SessionManagerLayer::new(store).with_secure(false)))
}

/// Reopens the database and checks the application can read the existing session.
#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let directory = tempfile::tempdir()?;
    let options = SqliteConnectOptions::new()
        .filename(directory.path().join("sessions.db"))
        .create_if_missing(true);
    let pool = SqlitePool::connect_with(options.clone()).await?;
    let response = app(pool.clone())
        .await?
        .oneshot(Request::post("/remember").body(Body::empty())?)
        .await?;
    let cookie = response
        .headers()
        .get(header::SET_COOKIE)
        .ok_or_else(|| anyhow::anyhow!("missing session cookie"))?
        .to_str()?
        .split(';')
        .next()
        .unwrap_or_default()
        .to_owned();
    pool.close().await;

    let pool = SqlitePool::connect_with(options).await?;
    let response = app(pool.clone())
        .await?
        .oneshot(
            Request::get("/theme")
                .header(header::COOKIE, cookie)
                .body(Body::empty())?,
        )
        .await?;
    assert_eq!(
        response.into_body().collect().await?.to_bytes().as_ref(),
        b"dark"
    );
    pool.close().await;
    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    /// Checks a new router and pool can read a session saved by the first router.
    #[test]
    fn persists_across_router_rebuild() -> anyhow::Result<()> {
        super::main()
    }
}
