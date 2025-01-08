// // ANCHOR: example
// #[get("/")]
// fn hello() -> &'static str {
//     "Hello, world!"
// }

// #[launch]
// fn rocket() -> _ {
//     rocket::build().mount("/", routes![hello])
// }
// // ANCHOR_END: example

// #[test]
// #[ignore = "WIP"]
// fn test() {
// }
// // [P1](https://github.com/john-cd/rust_howto/issues/869)
