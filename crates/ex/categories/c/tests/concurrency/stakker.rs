// ANCHOR: example

// Stakker is an actor-based system for Rust, which makes it great for building
// concurrent, distributed applications.
// use stakker::Deferrer;
// use stakker::Stakker;
// use stakker::StopCause;
// use stakker::actor;
// use stakker::stop;

// struct Hello;

// impl Hello {
//     fn greet(&self) {
//         println!("Hello, world!");
//     }
// }

// fn main() {
//     let mut stakker = Stakker::new(Deferrer::new());

//     actor!(stakker, Hello::greet(), async {
//         let actor = Hello;
//         actor.greet();
//         stop!(actor);
//     });

//     stakker.run(10_000_000);
// }
// ANCHOR_END: example

#[test]
#[ignore = "not implemented"]
fn test() {
    // main();
}
// [add stakker example](https://github.com/john-cd/rust_howto/issues/94)
