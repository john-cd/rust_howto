// // ANCHOR: example

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

// async fn graphql_handler(
//     schema: Extension<Schema<Query, EmptyMutation, EmptySubscription>>,
//     req: GraphQLRequest,
// ) -> GraphQLResponse {
//     schema.execute(req.into_inner()).await.into()
// }
// // ANCHOR_END: example

// #[test]
// fn test() {
//     main();
// }
// // [P1](https://github.com/john-cd/rust_howto/issues/864)
// // https://github.com/async-graphql/examples
