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

/// The application router; serve `openapi_json()` from a route as well,
/// or add `utoipa-swagger-ui` for an interactive UI.
pub fn app() -> Router {
    Router::new().route("/hello/{name}", get(hello))
}

/// Renders the document as pretty-printed JSON.
pub fn openapi_json() -> anyhow::Result<String> {
    Ok(ApiDoc::openapi().to_pretty_json()?)
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn document_describes_path_and_schema() -> anyhow::Result<()> {
        let doc: serde_json::Value = serde_json::from_str(&openapi_json()?)?;
        assert!(doc["paths"]["/hello/{name}"]["get"].is_object());
        assert!(doc["components"]["schemas"]["Greeting"].is_object());
        Ok(())
    }

    #[test]
    fn router_builds() {
        let _ = app();
    }
}
