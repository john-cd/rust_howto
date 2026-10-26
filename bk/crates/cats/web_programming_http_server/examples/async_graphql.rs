//! A small GraphQL endpoint using `async-graphql` with Axum.
//!
//! The `/graphql` route accepts GraphQL requests and runs them against a schema
//! with one `hello` query field. The optional `async-graphql-support` feature
//! enables this example.
//!
//! TODO Reviewed ??-??-2026.

// ANCHOR: example
use async_graphql::EmptyMutation;
use async_graphql::EmptySubscription;
use async_graphql::Object;
use async_graphql::Schema;
use async_graphql_axum::GraphQLRequest;
use async_graphql_axum::GraphQLResponse;
use axum::Router;
use axum::extract::State;
use axum::routing::post;
use tokio::net::TcpListener;

/// The root query object for the example schema.
struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Returns a greeting from the GraphQL endpoint.
    async fn hello(&self) -> &'static str {
        "Hello, world!"
    }
}

/// The GraphQL schema shared with the request handler.
type HelloSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

/// Executes a GraphQL request against the shared schema.
async fn graphql_handler(
    State(schema): State<HelloSchema>,
    request: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(request.into_inner()).await.into()
}

/// Creates the schema shared by the server and deterministic tests.
fn build_schema() -> HelloSchema {
    Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish()
}

/// Starts the GraphQL HTTP endpoint on the loopback interface.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    serve().await
}

/// Builds the GraphQL schema and serves it until the listener stops.
async fn serve() -> anyhow::Result<()> {
    let schema = build_schema();
    let app = Router::new()
        .route("/graphql", post(graphql_handler))
        .with_state(schema);

    let listener = TcpListener::bind("127.0.0.1:8000").await?;
    println!(
        "GraphQL endpoint listening on http://{}",
        listener.local_addr()?
    );
    axum::serve(listener, app).await?;
    Ok(())
}

// ANCHOR_END: example

pub fn run() -> anyhow::Result<()> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(serve())
}

/// Verifies the example schema with deterministic queries and no network
/// server.
#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::build_schema;

    /// Returns the greeting JSON for a valid GraphQL query.
    #[tokio::test]
    async fn hello_query_returns_expected_data() {
        let response = build_schema().execute("{ hello }").await;

        assert!(response.errors.is_empty(), "query should succeed");
        let data = response
            .data
            .into_json()
            .expect("GraphQL response data should be valid JSON");
        assert_eq!(data, json!({ "hello": "Hello, world!" }));
    }

    /// Reports one deterministic validation error for an unknown field.
    #[tokio::test]
    async fn unknown_field_returns_validation_error() {
        let response = build_schema().execute("{ missing }").await;

        assert_eq!(response.errors.len(), 1);
        assert!(response.errors[0].message.contains("Unknown field"));
    }
}
