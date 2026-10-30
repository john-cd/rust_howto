// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Cookie-based login with `axum-login`, in-memory sessions and permissions.
//!
//! ### Cookie-Based Login and Session Management
//! Cookie-based session management maintains user authentication state across
//! HTTP requests:
//! 1. Upon successful authentication (verifying username and password hash),
//!    the server creates a unique session record in a session store
//!    (`MemoryStore`, Redis, or SQL database) and associates it with the user
//!    ID.
//! 2. The server sends a HTTP `Set-Cookie` header containing an
//!    encrypted/signed session ID token back to the client browser.
//! 3. On subsequent requests, the browser automatically includes the session
//!    cookie in the `Cookie` header.
//! 4. Middleware (`axum-login`) extracts the session ID, loads the session and
//!    user permissions from the backend, and authorizes access to protected
//!    routes.
//!
//! Production notes: set `Secure`, `HttpOnly` and `SameSite` on the session
//! cookie (`SessionManagerLayer::with_secure` / `with_same_site`), keep
//! `HttpOnly` on, call `AuthSession::login` after authenticating (it cycles the
//! session ID, which prevents session fixation), and protect state-changing
//! forms against CSRF.

use std::collections::HashMap;
use std::collections::HashSet;
use std::convert::Infallible;
use std::sync::Arc;

use axum::Form;
use axum::Router;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::routing::post;
use axum_login::AuthManagerLayerBuilder;
use axum_login::AuthUser;
use axum_login::AuthnBackend;
use axum_login::AuthzBackend;
use axum_login::UserId;
use axum_login::login_required;
use axum_login::permission_required;
use serde::Deserialize;
use tower_sessions::MemoryStore;
use tower_sessions::SessionManagerLayer;

use super::hashing;

/// A user. The `AuthUser` hash invalidates sessions when the password changes.
#[derive(Clone, Debug)]
pub struct User {
    id: i64,
    username: String,
    password_hash: String,
    permissions: HashSet<String>,
}

impl AuthUser for User {
    type Id = i64;

    fn id(&self) -> i64 {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.password_hash.as_bytes()
    }
}

/// Credentials posted by the login form.
#[derive(Clone, Debug, Deserialize)]
pub struct Credentials {
    username: String,
    password: String,
}

/// An in-memory user store. Replace with a database in a real application.
#[derive(Clone)]
pub struct Backend {
    users: Arc<HashMap<i64, User>>,
    /// Hash verified when the username is unknown, so that the response time
    /// does not reveal which usernames exist.
    dummy_hash: Arc<String>,
}

impl Backend {
    pub fn new(users: Vec<User>) -> anyhow::Result<Self> {
        Ok(Self {
            users: Arc::new(users.into_iter().map(|u| (u.id, u)).collect()),
            dummy_hash: Arc::new(hashing::hash("dummy password")?),
        })
    }
}

impl AuthnBackend for Backend {
    type Credentials = Credentials;
    type Error = Infallible;
    type User = User;

    async fn authenticate(
        &self,
        creds: Credentials,
    ) -> Result<Option<User>, Infallible> {
        let user = self.users.values().find(|u| u.username == creds.username);
        // Always run one verification, whether or not the user exists.
        let stored =
            user.map_or(self.dummy_hash.as_str(), |u| &u.password_hash);
        let ok = hashing::verify(&creds.password, stored);
        Ok(user.filter(|_| ok).cloned())
    }

    async fn get_user(
        &self,
        id: &UserId<Self>,
    ) -> Result<Option<User>, Infallible> {
        Ok(self.users.get(id).cloned())
    }
}

impl AuthzBackend for Backend {
    type Permission = String;

    async fn get_user_permissions(
        &self,
        user: &User,
    ) -> Result<HashSet<String>, Infallible> {
        Ok(user.permissions.clone())
    }
}

type AuthSession = axum_login::AuthSession<Backend>;

async fn login(
    mut session: AuthSession,
    Form(creds): Form<Credentials>,
) -> impl IntoResponse {
    match session.authenticate(creds).await {
        Ok(Some(user)) => match session.login(&user).await {
            Ok(()) => StatusCode::OK,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        },
        Ok(None) => StatusCode::UNAUTHORIZED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

async fn protected(session: AuthSession) -> String {
    match session.user {
        Some(user) => format!("Hello, {}!", user.username),
        None => "Hello, stranger!".to_owned(),
    }
}

async fn admin() -> &'static str {
    "Admin area"
}

/// Builds the application.
pub fn app(backend: Backend) -> Router {
    // In-memory sessions: lost on restart. Use a database-backed store in
    // production, e.g. `tower-sessions-sqlx-store`.
    let session_layer =
        SessionManagerLayer::new(MemoryStore::default()).with_secure(false); // `true` when served over HTTPS.
    let auth_layer =
        AuthManagerLayerBuilder::new(backend, session_layer).build();

    Router::new()
        .route("/admin", get(admin))
        .route_layer(permission_required!(Backend, "admin.read"))
        .route("/protected", get(protected))
        .route_layer(login_required!(Backend, login_url = "/login"))
        .route("/login", post(login))
        .layer(auth_layer)
}
// ANCHOR_END: example

/// Demonstrates initialization of the axum_login application backend.
pub fn run() -> anyhow::Result<()> {
    let user = User {
        id: 1,
        username: "alice".to_owned(),
        password_hash: hashing::hash("secret")?,
        permissions: std::iter::once("admin.read".to_owned()).collect(),
    };
    let _router = app(Backend::new(vec![user])?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::Request;
    use axum::http::Response;
    use axum::http::header;
    use http_body_util::BodyExt;
    use tower::ServiceExt as _;

    use super::*;

    fn test_app() -> anyhow::Result<Router> {
        let user = |id, name: &str, perms: &[&str]| -> anyhow::Result<User> {
            Ok(User {
                id,
                username: name.to_owned(),
                password_hash: hashing::hash("secret")?,
                permissions: perms.iter().map(|p| (*p).to_owned()).collect(),
            })
        };
        Ok(app(Backend::new(vec![
            user(1, "alice", &["admin.read"])?,
            user(2, "bob", &[])?,
        ])?))
    }

    async fn login_as(
        app: &Router,
        name: &str,
        pw: &str,
    ) -> anyhow::Result<Response<Body>> {
        let request = Request::post("/login")
            .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
            .body(Body::from(format!("username={name}&password={pw}")))?;
        Ok(app.clone().oneshot(request).await?)
    }

    fn cookie(response: &Response<Body>) -> Option<String> {
        let value =
            response.headers().get(header::SET_COOKIE)?.to_str().ok()?;
        value.split(';').next().map(str::to_owned)
    }

    async fn get_with(
        app: &Router,
        uri: &str,
        cookie: Option<&str>,
    ) -> anyhow::Result<Response<Body>> {
        let mut builder = Request::get(uri);
        if let Some(c) = cookie {
            builder = builder.header(header::COOKIE, c);
        }
        Ok(app.clone().oneshot(builder.body(Body::empty())?).await?)
    }

    #[tokio::test]
    async fn unauthenticated_is_redirected() -> anyhow::Result<()> {
        let app = test_app()?;
        let response = get_with(&app, "/protected", None).await?;
        assert_eq!(response.status(), StatusCode::TEMPORARY_REDIRECT);
        Ok(())
    }

    #[tokio::test]
    async fn wrong_or_unknown_credentials_are_rejected() -> anyhow::Result<()> {
        let app = test_app()?;
        assert_eq!(
            login_as(&app, "alice", "nope").await?.status(),
            StatusCode::UNAUTHORIZED
        );
        assert_eq!(
            login_as(&app, "mallory", "secret").await?.status(),
            StatusCode::UNAUTHORIZED
        );
        Ok(())
    }

    #[tokio::test]
    async fn login_then_access() -> anyhow::Result<()> {
        let app = test_app()?;
        let response = login_as(&app, "alice", "secret").await?;
        assert_eq!(response.status(), StatusCode::OK);
        let session =
            cookie(&response).ok_or_else(|| anyhow::anyhow!("no cookie"))?;

        let response = get_with(&app, "/protected", Some(&session)).await?;
        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await?.to_bytes();
        assert_eq!(&*body, b"Hello, alice!");
        Ok(())
    }

    #[tokio::test]
    async fn permission_is_required_for_admin() -> anyhow::Result<()> {
        let app = test_app()?;
        let alice = cookie(&login_as(&app, "alice", "secret").await?)
            .ok_or_else(|| anyhow::anyhow!("no cookie"))?;
        let bob = cookie(&login_as(&app, "bob", "secret").await?)
            .ok_or_else(|| anyhow::anyhow!("no cookie"))?;

        assert_eq!(
            get_with(&app, "/admin", Some(&alice)).await?.status(),
            StatusCode::OK
        );
        assert_eq!(
            get_with(&app, "/admin", Some(&bob)).await?.status(),
            StatusCode::FORBIDDEN
        );
        Ok(())
    }
}
