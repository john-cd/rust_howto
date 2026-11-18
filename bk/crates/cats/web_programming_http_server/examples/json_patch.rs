//! A local Axum PATCH endpoint with absent/null/value field semantics.

use std::sync::Arc;
use std::sync::Mutex;

use axum::Router;
use axum::extract::Json;
use axum::extract::State;
use axum::extract::rejection::JsonRejection;
use axum::http::StatusCode;
use axum::routing::patch;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;

// ANCHOR: example
/// Preserves the distinction between an omitted JSON field and explicit null.
#[derive(Default, Debug)]
enum Field<T> {
    /// No key was supplied; retain the stored value through `serde(default)`.
    #[default]
    Absent,
    /// An explicit JSON null requests clearing the stored value.
    Null,
    /// A non-null JSON value requests replacing the stored value.
    Value(
        /// Replacement value carried by this JSON patch operation.
        T,
    ),
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Field<T> {
    /// Maps a present JSON value to replacement or clear; missing keys use Default.
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(match Option::<T>::deserialize(deserializer)? {
            Some(value) => Self::Value(value),
            None => Self::Null,
        })
    }
}

/// Incoming partial update whose missing fields leave the profile unchanged.
#[derive(Deserialize)]
struct ProfilePatch {
    /// Requested nickname change: retain, clear, or replace the stored text.
    #[serde(default)]
    nickname: Field<String>,
}

/// Stored profile serialized after a successful partial update.
#[derive(Clone, Serialize)]
struct Profile {
    /// Current nickname, or None when no nickname is configured.
    nickname: Option<String>,
}

/// Shared in-memory state protected during a synchronous profile mutation.
type SharedProfile = Arc<Mutex<Profile>>;
/// HTTP error status paired with a JSON object containing an error message.
type ApiError = (StatusCode, Json<serde_json::Value>);

/// Builds the common JSON error envelope with the supplied HTTP status.
fn api_error(status: StatusCode, message: &str) -> ApiError {
    (status, Json(serde_json::json!({ "error": message })))
}

/// Applies a validated nickname patch while holding the profile lock.
///
/// Rejects malformed JSON with 400, empty or over-32-byte names with 422, and a
/// poisoned state lock with 500. Validation precedes mutation; omitted keys
/// retain state, null clears it, and strings replace it.
async fn update(
    State(profile): State<SharedProfile>,
    body: Result<Json<ProfilePatch>, JsonRejection>,
) -> Result<Json<Profile>, ApiError> {
    let Json(body) = body.map_err(|_| api_error(StatusCode::BAD_REQUEST, "invalid JSON body"))?;
    if let Field::Value(ref name) = body.nickname
        && (name.is_empty() || name.len() > 32)
    {
        return Err(api_error(
            StatusCode::UNPROCESSABLE_ENTITY,
            "nickname must be 1..=32 bytes",
        ));
    }
    let mut current = profile
        .lock()
        .map_err(|_| api_error(StatusCode::INTERNAL_SERVER_ERROR, "state unavailable"))?;
    match body.nickname {
        Field::Absent => {}
        Field::Null => current.nickname = None,
        Field::Value(name) => current.nickname = Some(name),
    }
    Ok(Json(current.clone()))
}

/// Wires PATCH /profile to the supplied shared profile without starting a server.
fn app(profile: SharedProfile) -> Router {
    Router::new()
        .route("/profile", patch(update))
        .with_state(profile)
}
// ANCHOR_END: example

/// Builds a sample router and prints the command for its in-process tests.
fn main() {
    let _router = app(Arc::new(Mutex::new(Profile {
        nickname: Some("Ada".into()),
    })));
    println!(
        "Test the router with: cargo test -p web_programming_http_server --example json_patch"
    );
}

/// Exercises patch semantics and rejection paths through the in-process router.
#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::body::to_bytes;
    use axum::http::Request;
    use tower::ServiceExt;

    use super::*;

    /// Sends a JSON PATCH through the router and collects its status and JSON body.
    async fn request(router: Router, body: &str) -> (StatusCode, serde_json::Value) {
        let response = router
            .oneshot(
                Request::patch("/profile")
                    .header("content-type", "application/json")
                    .body(Body::from(body.to_owned()))
                    .expect("request"),
            )
            .await
            .expect("router response");
        let status = response.status();
        let bytes = to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("body bytes");
        (
            status,
            serde_json::from_slice(&bytes).expect("JSON response"),
        )
    }

    /// Checks that distinguishes absent null and value.
    #[tokio::test]
    async fn distinguishes_absent_null_and_value() {
        let router = app(Arc::new(Mutex::new(Profile {
            nickname: Some("Ada".into()),
        })));
        let (status, body) = request(router.clone(), "{}").await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body["nickname"], "Ada");

        let (status, body) = request(router.clone(), r#"{"nickname":null}"#).await;
        assert_eq!(status, StatusCode::OK);
        assert!(body["nickname"].is_null());

        let (status, body) = request(router, r#"{"nickname":"Grace"}"#).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body["nickname"], "Grace");
    }

    /// Checks that rejects invalid payloads.
    #[tokio::test]
    async fn rejects_invalid_payloads() {
        let router = app(Arc::new(Mutex::new(Profile { nickname: None })));
        let (status, body) = request(router.clone(), "not JSON").await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert_eq!(body["error"], "invalid JSON body");

        let (status, body) = request(router, r#"{"nickname":""}"#).await;
        assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
        assert_eq!(body["error"], "nickname must be 1..=32 bytes");
    }
}
