// // ANCHOR: example
// // ANCHOR_END: example

// use ractor::Actor;
// use ractor::ActorProcessingErr;
// use ractor::ActorRef;
// use tokio::time::Duration;
// use tokio::time::sleep;

// // `ractor` is a pure-Rust actor framework.

// // Define a message type
// struct Greet;

// // Define an actor
// struct Greeter;

// #[async_trait::async_trait]
// impl ractor::Actor for Greeter {
//     type Msg = Greet;
//     type State = ();

//     async fn pre_start(&self, _ctx: &ractor::ActorContext<Self::Msg>) ->
// Self::State {         println!("Greeter actor is starting");
//         ()
//     }

//     async fn handle(&self, _ctx: &ractor::ActorContext<Self::Msg>, _msg:
// Self::Msg, _state: &mut Self::State) {         println!("Received a Greet
// message");     }

//     // async fn handle_rpc(&self, _ctx: &ractor::ActorContext<Self::Msg>,
// _msg: Self::Msg, _state: &mut Self::State, reply: RpcReplyPort<String>) {
//     //     reply.send("Hello from Greeter!".to_string()).unwrap();
//     // }

//     async fn post_stop(&self, _ctx: &ractor::ActorContext<Self::Msg>, _state:
// &mut Self::State) {         println!("Greeter actor is stopping");
//     }
// }

// #[tokio::main]
// async fn main() {
//     // Create a Greeter actor
//     let (greeter, handle) = Actor::spawn(None, Greeter).await.expect("Failed
// to spawn Greeter");

//     // Send a Greet message and wait for the reply
//     let result = greeter.call(Greet).await.unwrap();
//     println!("Greeter replied: {}", result);

//     // Sleep for a bit to let the actor finish any remaining work
//     sleep(Duration::from_secs(1)).await;

//     // Stop the actor
//     handle.stop().await.expect("Failed to stop Greeter");
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use tokio_test::block_on;

//     #[test]
//     fn test() {
//         block_on(main());
//     }
// }
// // [P1](https://github.com/john-cd/rust_howto/issues/685)
