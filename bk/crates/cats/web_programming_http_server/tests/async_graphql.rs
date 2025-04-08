// // ANCHOR: example
// // COMING SOON
// // ANCHOR_END: example
// //! # Example: Async GraphQL with Axum
// //!
// //! This example demonstrates how to set up a simple GraphQL server using the
// //! `async-graphql` and `axum` crates.
// //!
// //! It defines a basic query that returns "Hello, world!" and sets up an Axum
// //! server to handle GraphQL requests.

// use async_graphql::Context;
// use async_graphql::EmptyMutation;
// use async_graphql::EmptySubscription;
// use async_graphql::Object;
// use async_graphql::Schema;
// use async_graphql_axum::GraphQLRequest;
// use async_graphql_axum::GraphQLResponse;
// use axum::Router;
// use axum::extract::Extension;
// use axum::routing::post;

// struct Query;

// #[Object]
// impl Query {
//     async fn hello(&self, _ctx: &Context<'_>) -> &str {
//         "Hello, world!"
//     }
// }

// #[tokio::main]
// async fn main() {
//     // Create the schema
//     let schema =
//         Schema::build(Query, EmptyMutation, EmptySubscription).finish();

//     // Set up the Axum app with the GraphQL endpoint
//     let app = Router::new()
//         .route("/graphql", post(graphql_handler))
//         .layer(Extension(schema));

//     // Run the Axum server
//     println!("GraphQL server started at http://localhost:8000");
//     Server::bind(&"127.0.0.1:8000".parse().unwrap())
//         .serve(app.into_make_service())
//         .await
//         .unwrap();
// }

// /// Handles incoming GraphQL requests.
// ///
// /// This function extracts the GraphQL schema from the Axum extension and the
// /// GraphQL request from the request body. It then executes the GraphQL
// /// request against the schema and returns the response.
// ///
// /// # Arguments
// ///
// /// * `schema` - The GraphQL schema, extracted from the Axum extension.
// /// * `req` - The GraphQL request, extracted from the request body.
// async fn graphql_handler(
//     schema: Extension<Schema<Query, EmptyMutation, EmptySubscription>>,
//     req: GraphQLRequest,
// ) -> GraphQLResponse {
//     schema.execute(req.into_inner()).await.into()
// }

// #[test]
// fn require_network() {
//     main();
// }
// // [finish;  https://github.com/async-graphql/examples](https://github.com/john-cd/rust_howto/issues/864)
