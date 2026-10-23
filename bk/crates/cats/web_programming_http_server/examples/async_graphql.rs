//! A small GraphQL endpoint using `async-graphql` with Axum.
//!
//! The `/graphql` route accepts GraphQL requests and runs them against a schema
//! with one `hello` query field. The optional `async-graphql-support` feature
//! enables this example.
//!
//! TODO Reviewed ??-??-2026.

// // ANCHOR: example
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

/// Starts the GraphQL HTTP endpoint on the loopback interface.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    serve().await
}

/// Builds the GraphQL schema and serves it until the listener stops.
async fn serve() -> anyhow::Result<()> {
    let schema =
        Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish();
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

/// Runs the example.
pub fn run() -> anyhow::Result<()> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(serve())
}

// [finish; https://github.com/async-graphql/examples](https://github.com/john-cd/rust_howto/issues/864)
