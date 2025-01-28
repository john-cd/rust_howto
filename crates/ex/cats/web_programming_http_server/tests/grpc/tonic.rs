// // ANCHOR: example
// COMING SOON
// // ANCHOR_END: example

// use tonic::{transport::Server, Request, Response, Status};
// use helloworld::greeter_server::{Greeter, GreeterServer};
// use helloworld::{HelloRequest, HelloReply};
// use tonic::server::NamedService;

// // Tonic is a gRPC library.

// pub mod helloworld {
//     tonic::include_proto!("helloworld");
// }

// #[derive(Debug, Default)]
// pub struct MyGreeter {}

// #[tonic::async_trait]
// impl Greeter for MyGreeter {
//     async fn say_hello(
//         &self,
//         request: Request<HelloRequest>,
//     ) -> Result<Response<HelloReply>, Status> {
//         let reply = helloworld::HelloReply {
//             message: format!("Hello {}!", request.into_inner().name),
//         };

//         Ok(Response::new(reply))
//     }
// }

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let addr = "[::1]:50051".parse().unwrap();
//     let greeter = MyGreeter::default();

//     Server::builder()
//         .add_service(GreeterServer::new(greeter))
//         .serve(addr)
//         .await?;

//     Ok(())
// }

// #[test]
// fn test() {
//     main();
// }
// // [P1](https://github.com/john-cd/rust_howto/issues/870)
