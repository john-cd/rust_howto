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
#[derive(Default, Debug)]
enum Field<T> {
    /// Represents the absent case of field.
    #[default]
    Absent,
    /// Represents the null case of field.
    Null,
    /// Represents the value case of field.
    Value(T),
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Field<T> {
    /// Handles the deserialize behavior for this component.
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(match Option::<T>::deserialize(deserializer)? {
            Some(value) => Self::Value(value),
            None => Self::Null,
        })
    }
}

/// Data for profile patch operations, including nickname.
#[derive(Deserialize)]
struct ProfilePatch {
    /// Nickname stored for this value.
    #[serde(default)]
    nickname: Field<String>,
}

/// Data for profile operations, including nickname.
#[derive(Clone, Serialize)]
struct Profile {
    /// Nickname stored for this value.
    nickname: Option<String>,
}

/// Alias for `Arc<Mutex<Profile>>` used to represent shared profile.
type SharedProfile = Arc<Mutex<Profile>>;
/// Alias for `(StatusCode, Json<serde_json::Value>)` used to represent api error.
type ApiError = (StatusCode, Json<serde_json::Value>);

/// Handles the api error behavior for this component.
fn api_error(status: StatusCode, message: &str) -> ApiError {
    (status, Json(serde_json::json!({ "error": message })))
}

/// Handles the update behavior for this component.
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

/// Handles the app behavior for this component.
fn app(profile: SharedProfile) -> Router {
    Router::new()
        .route("/profile", patch(update))
        .with_state(profile)
}
// ANCHOR_END: example

fn main() {
    let _router = app(Arc::new(Mutex::new(Profile {
        nickname: Some("Ada".into()),
    })));
    println!(
        "Test the router with: cargo test -p web_programming_http_server --example json_patch"
    );
}

/// Tests behavior implemented by the surrounding module.
#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::body::to_bytes;
    use axum::http::Request;
    use tower::ServiceExt;

    use super::*;

    /// Handles the request behavior for this component.
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
