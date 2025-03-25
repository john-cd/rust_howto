// // ANCHOR: example
// // COMING SOON
// // ANCHOR_END: example
// use frunk::HList;
// use frunk::Plucker;
// use frunk::Sculptor;
// use frunk::hlist::HCons;
// use frunk::hlist::HNil;

// // Defining a struct with generic type
// #[derive(Debug, Clone)]
// struct User<T> {
//     id: T,
//     name: String,
// }

// fn main() {
//     // Heterogeneous List (HList) with mixed types
//     let data = HCons(42, HCons("Hello".to_string(), HCons(3.14, HNil)));

//     // Pattern matching on HList
//     match data {
//         HCons(num, HCons(text, HCons(float, HNil))) => {
//             println!("Number: {}, Text: {}, Float: {}", num, text, float);
//         }
//         _ => println!("Unexpected list structure"),
//     }

//     // Generic User creation
//     let user_int = User {
//         id: 100,
//         name: "Alice".to_string(),
//     };
//     let user_str = User {
//         id: "U12345",
//         name: "Bob".to_string(),
//     };

//     // Type-level plucking
//     let data_list = HCons(user_int, HCons(user_str, HNil));

//     // Pluck out specific types from HList
//     let (int_users, str_users): (Vec<User<i32>>, Vec<User<String>>) =
//         data_list.pluck();

//     println!("Int ID Users: {:?}", int_users);
//     println!("String ID Users: {:?}", str_users);

//     // Sculptor trait for transforming HLists
//     let shortened_list = data_list.sculpt::<(User<String>, User<i32>)>();

//     println!("Transformed list: {:?}", shortened_list);
// }

// #[test]
// fn test() {
//     main();
// }
// // [finish example](https://github.com/john-cd/rust_howto/issues/1318)
