// // ANCHOR: example

// // This will serialize and then deserialize a Person protobuf message and
// // print out its contents.

// mod person {
//     include!(concat!(env!("OUT_DIR"), "/tests/proto/person.rs"));
// }

// use person::Person;
// use prost::Message;

// fn main() {
//     // Create a Person message
//     let person = Person {
//         name: "Alice".to_string(),
//         age: 30,
//     };

//     // Serialize the message
//     let mut buf = Vec::new();
//     person.encode(&mut buf).unwrap();

//     // Deserialize the message
//     let decoded_person = Person::decode(&*buf).unwrap();

//     println!("Name: {}, Age: {}", decoded_person.name, decoded_person.age);
// }
// // ANCHOR_END: example

// #[test]
// fn test() {
//     main();
// }
// [ P1 finish prost example](https://github.com/john-cd/rust_howto/issues/1044)
