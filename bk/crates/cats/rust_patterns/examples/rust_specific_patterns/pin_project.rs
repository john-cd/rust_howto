#![allow(dead_code)]
// // ANCHOR: example
// //! The following demonstrates the use of the `pin_project` crate for pin
// //! projection.
// //!
// //! Pin projection enables safely getting mutable references to fields of a
// //! `Pin`-ned struct.
// //!
// //! Pinning is a mechanism that guarantees that the memory location of a
// //! value will not change. This is useful when there are one or more
// //! pointers pointing at that value, for example in self-referential data
// //! structures, asynchronous programming, and `unsafe` code.
// //! See [`std::pin`](https://doc.rust-lang.org/std/pin/index.html).
// //!
// //! Projection refers to allow borrowing one or more of the inner fields of a
// //! struct when the caller has access to a borrow of the whole struct.
// //!
// //! `pin-project` enables safe [projection and structural pinning](https://doc.rust-lang.org/std/pin/index.html#projections-and-structural-pinning).

// use std::fmt::Debug;
// use std::marker::PhantomPinned;
// use std::pin::Pin;

// use pin_project::pin_project;
// use pin_project::pinned_drop;

// /// The `#[pin_project]` attribute creates a projection type covering all the
// /// fields of struct or enum.
// ///
// /// - For the fields that use #[pin] attribute, it creates the pinned
// reference ///   to the field.
// /// - For the other fields, it creates a normal reference to the field.
// ///
// /// This works for tuple structs as well.
// #[pin_project]
// struct BasicPinned<T, U>
// where
//     T: Debug,
//     U: Clone,
// {
//     #[pin]
//     pinned: T,
//     unpinned: U,
// }

// /// `#[pin_project]` also implements the following methods on the original
// type: /// `fn project(self: Pin<&mut Self>) -> Projection<'_>;`
// /// `fn project_ref(self: Pin<&Self>) -> ProjectionRef<'_>;`
// #[allow(dead_code)]
// impl<T: Debug, U: Clone> BasicPinned<T, U> {
//     fn method(self: Pin<&mut Self>) {
//         // Return the projection type:
//         let this = self.project();
//         let _: Pin<&mut T> = this.pinned; // Pinned reference to the field.
//         let _: &mut U = this.unpinned; // Normal reference to the field.
//     }
// }

// // FIXME make use of project()
// // Example 1: Basic pinned struct.
// fn example1() {
//     let mut basic = BasicPinned {
//         pinned: String::from("pinned data"),
//         unpinned: 42,
//     };

//     let mut pinned_basic = unsafe { Pin::new_unchecked(&mut basic) };
//     let projection = pinned_basic.as_mut().project();

//     // `projection.pinned` is `Pin<&mut String>`.
//     // `projection.unpinned` is `&mut i32`.
//     *projection.unpinned += 1;
//     println!("Updated unpinned value: {}", projection.unpinned);
// }

// /// ## Self-Referential Struct
// ///
// /// This struct demonstrates how to create a self-referential struct using
// /// `#[pin_project]`.
// ///
// /// Linked Lists, Trees with Parent Pointers
// #[pin_project]
// struct SelfReferential {
//     #[pin]
//     value: String,
//     // `pointer` will refer to the `value` above, making this type
//     // self-referential.
//     pointer: *const String,

//     // The vast majority of Rust types have no address-sensitive states.
//     // These types implement the `Unpin` auto-trait, which cancels the
//     // restrictive effects of `Pin` when the pointee type `T` is `Unpin`.
//     // In this example, we rely upon pinning for soundness and must ensure
//     // that the type is not automatically marked as `Unpin`. We do so by
//     // adding a `PhantomPinned` marker field.
//     #[pin] // <------ This `#[pin]` is required to make `Struct` to `!Unpin`.
//     _marker: PhantomPinned,
//     // This is equivalent to `#[pin_project(!Unpin)]`.
// }

// impl SelfReferential {
//     fn new(value: String) -> Self {
//         Self {
//             value,
//             pointer: std::ptr::null(),
//             _marker: PhantomPinned,
//         }
//     }

//     fn init(self: Pin<&mut Self>) {
//         let this = self.project();
//         let ptr = this.value.as_ptr();
//         *this.pointer = ptr as *const String;
//     }

//     fn get_pointer(self: Pin<&Self>) -> *const String {
//         self.pointer
//     }

//     // FIXME
//     // fn get_value(self: Pin<&Self>) -> &String {
//     //     &self.value
//     // }
// }

// // FIXME make use of project()
// // Example 2: Self-referential struct.
// fn example2() {
//     let mut self_ref =
//         SelfReferential::new(String::from("self-referential data"));

//     // Pin the struct to the stack.
//     let mut pinned_self_ref = unsafe { Pin::new_unchecked(&mut self_ref) };

//     // Initialize the self-reference.
//     pinned_self_ref.as_mut().init();

//     // Access the data through the self-reference.
//     let ptr = pinned_self_ref.as_ref().get_pointer();
//     // let value = pinned_self_ref.as_ref().get_value();

//     // println!("Value: {value}");
//     println!("Pointer points to value: {}", unsafe { &*ptr });
// }

// /// ## Pinned Enum
// ///
// /// This enum demonstrates how to use `pin_project`
// /// with an enum.
// ///
// /// By passing an argument with the same name as the projection method
// /// to the `pin_project` attribute, we can name the projection type returned
// /// from the method. This allows us to use pattern matching on the projected
// /// types.
// #[pin_project(project = EnumProj)]
// enum PinnedEnum<T, U> {
//     Variant1(#[pin] T, U),
// }

// impl<T: Debug, U: Debug> PinnedEnum<T, U> {
//     fn process(self: Pin<&mut Self>) {
//         match self.project() {
//             // Note the named projection type:
//             EnumProj::Variant1(pinned, unpinned) => {
//                 println!(
//                     "Processing Variant1 with pinned {:?} and unpinned:
// {:?}",                     pinned, unpinned
//                 );
//                 // `pinned` is `Pin<&mut T>`,
//                 // `unpinned` is `&mut U`.
//             }
//         }
//     }
// }

// // FIXME make use of project()
// // Example 3: Enum.
// fn example3() {
//     let mut enum_value =
//         PinnedEnum::Variant1(String::from("pinned enum data"), 123);
//     let mut pinned_enum = unsafe { Pin::new_unchecked(&mut enum_value) };

//     pinned_enum.as_mut().process();
// }

// /// ## Pinned Struct with Custom Drop.
// ///
// /// In order to correctly implement pin projections, a type's `Drop` impl
// must /// not move out of any structurally pinned fields.
// ///
// /// This struct demonstrates how to use `#[pin_project(PinnedDrop)]` to
// /// create a struct with a custom `drop` implementation.
// #[pin_project(PinnedDrop)]
// struct PinnedWithDrop<T: Debug> {
//     #[pin]
//     pinned_field: T,
//     unpinned_field: String,
// }

// /// The #[pin_project] attribute will provide a `Drop` implementation,
// /// based on your implementation of the `PinnedDrop` trait:
// #[pinned_drop]
// impl<T: Debug> PinnedDrop for PinnedWithDrop<T> {
//     fn drop(self: Pin<&mut Self>) {
//         // this method takes `Pin<&mut Self>`.
//         println!(
//             "Dropping PinnedWithDrop with pinned_field {:?} and
// unpinned_field: {}",             self.pinned_field, self.unpinned_field
//         );
//     }
// }

// // Example 4: Custom `drop`.
// fn example4() {
//     let _pinned_drop = PinnedWithDrop {
//         pinned_field: 42,
//         unpinned_field: String::from("will be dropped"),
//     };

//     println!("Created PinnedWithDrop, will be dropped at end of scope");
// }

// fn main() {
//     example1();
//     example2();
//     example3();
//     example4();
// }
// // ANCHOR_END: example

// #[test]
// fn test() {
//     main();
// }
// // [finish; provide projection example before going into pin projection; explain risks; explain use cases](https://github.com/john-cd/rust_howto/issues/1120)
