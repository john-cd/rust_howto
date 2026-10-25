#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Generate an OpenAPI document from code with [`utoipa`].

use axum::Json;
use axum::Router;
use axum::extract::Path;
use axum::routing::get;
use serde::Serialize;
use utoipa::OpenApi;
use utoipa::ToSchema;
use utoipa_swagger_ui::SwaggerUi;

/// A greeting.
#[derive(Serialize, ToSchema)]
pub struct Greeting {
    /// The text of the greeting.
    message: String,
}

/// Greets `name`.
#[utoipa::path(
    get,
    path = "/hello/{name}",
    params(("name" = String, Path, description = "Who to greet")),
    responses((status = 200, description = "A greeting", body = Greeting))
)]
async fn hello(Path(name): Path<String>) -> Json<Greeting> {
    Json(Greeting {
        message: format!("Hello, {name}!"),
    })
}

/// The OpenAPI document.
#[derive(OpenApi)]
#[openapi(paths(hello), components(schemas(Greeting)))]
pub struct ApiDoc;

/// The application router. `SwaggerUi` serves the interactive UI at
/// `/swagger-ui` and the generated document at `/api-docs/openapi.json`.
pub fn app() -> Router {
    Router::new().route("/hello/{name}", get(hello)).merge(
        SwaggerUi::new("/swagger-ui")
            .url("/api-docs/openapi.json", ApiDoc::openapi()),
    )
}

/// Renders the document as pretty-printed JSON.
pub fn openapi_json() -> anyhow::Result<String> {
    Ok(ApiDoc::openapi().to_pretty_json()?)
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::Request;
    use axum::http::StatusCode;
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    use super::*;

    #[test]
    fn document_describes_path_and_schema() -> anyhow::Result<()> {
        let doc: serde_json::Value = serde_json::from_str(&openapi_json()?)?;
        assert!(doc["paths"]["/hello/{name}"]["get"].is_object());
        assert!(doc["components"]["schemas"]["Greeting"].is_object());
        Ok(())
    }

    async fn get_status_and_body(uri: &str) -> (StatusCode, Vec<u8>) {
        let response = app()
            .oneshot(Request::get(uri).body(Body::empty()).unwrap())
            .await
            .unwrap();
        let status = response.status();
        let body = response.into_body().collect().await.unwrap().to_bytes();
        (status, body.to_vec())
    }

    #[tokio::test]
    async fn serves_the_spec_from_a_route() {
        let (status, body) =
            get_status_and_body("/api-docs/openapi.json").await;
        assert_eq!(status, StatusCode::OK);
        let doc: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert!(doc["paths"]["/hello/{name}"].is_object());
    }

    #[tokio::test]
    async fn serves_the_swagger_ui() {
        let (status, body) = get_status_and_body("/swagger-ui/").await;
        assert_eq!(status, StatusCode::OK);
        assert!(String::from_utf8_lossy(&body).contains("swagger"));
    }
}
