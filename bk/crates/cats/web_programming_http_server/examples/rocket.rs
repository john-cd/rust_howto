// ANCHOR: example
// // COMING SOON
// //! This is a simple example of a Rocket web server.
// //!
// //! It defines a single route that returns "Hello, world!" when accessed.

// #[get("/")]
// fn hello() -> &'static str {
//     "Hello, world!"
// }

// #[launch]
// fn rocket() -> _ {
//     rocket::build().mount("/", routes![hello])
// }

// // [Implement Rocket Hello World and route mounting](https://github.com/john-cd/rust_howto/issues/869)

fn main() {
    // TODO
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
// TODO implement the Rocket example; see https://github.com/john-cd/rust_howto/issues/869
