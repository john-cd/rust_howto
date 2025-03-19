// // ANCHOR: example
// // COMING SOON
// // ANCHOR_END: example
// use std::fmt::Debug;
// use std::marker::PhantomPinned;
// use std::pin::Pin;

// use pin_project::pin_project;
// use pin_project::pinned_drop;

// // Basic example: a struct with a field that requires pinning
// #[pin_project]
// struct BasicPinned<T, U> {
//     #[pin]
//     pinned: T,
//     unpinned: U,
// }

// // Example with type parameters and where clauses
// #[pin_project]
// struct GenericPinned<T, U>
// where
//     T: Debug,
//     U: Clone,
// {
//     #[pin]
//     pinned_field: T,
//     unpinned_field: U,
// }

// // Example with custom drop implementation
// #[pin_project(PinnedDrop)]
// struct PinnedWithDrop<T> {
//     #[pin]
//     pinned_field: T,
//     unpinned_field: String,
// }

// #[pinned_drop]
// impl<T> PinnedDrop for PinnedWithDrop<T> {
//     fn drop(self: Pin<&mut Self>) {
//         println!(
//             "Dropping PinnedWithDrop with unpinned_field: {}",
//             self.unpinned_field
//         );
//     }
// }

// // Example with a self-referential struct
// #[pin_project]
// struct SelfReferential {
//     #[pin]
//     value: String,
//     pointer: *const String,
//     _marker: PhantomPinned,
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
//         let pointer = this.value.as_ptr();
//         unsafe {
//             *this.pointer = pointer;
//         }
//     }

//     fn get_pointer(self: Pin<&Self>) -> *const String {
//         self.pointer
//     }

//     fn get_value(self: Pin<&Self>) -> &String {
//         &self.value
//     }
// }

// // Example with projection and enum
// #[pin_project(project = EnumProj)]
// enum PinnedEnum<T, U> {
//     Variant1(#[pin] T, U),
//     Variant2(#[pin] T),
//     Variant3(U),
// }

// impl<T, U> PinnedEnum<T, U> {
//     fn process(self: Pin<&mut Self>) {
//         match self.project() {
//             EnumProj::Variant1(pinned, unpinned) => {
//                 println!("Processing Variant1 with unpinned: {:?}",
// unpinned);                 // pinned is Pin<&mut T>
//                 // unpinned is &mut U
//             }
//             EnumProj::Variant2(pinned) => {
//                 // pinned is Pin<&mut T>
//                 println!("Processing Variant2");
//             }
//             EnumProj::Variant3(unpinned) => {
//                 // unpinned is &mut U
//                 println!("Processing Variant3 with unpinned: {:?}",
// unpinned);             }
//         }
//     }
// }

// fn main() {
//     // Example 1: Basic pinned struct
//     let mut basic = BasicPinned {
//         pinned: String::from("pinned data"),
//         unpinned: 42,
//     };

//     let mut pinned_basic = unsafe { Pin::new_unchecked(&mut basic) };
//     let projection = pinned_basic.as_mut().project();

//     // projection.pinned is Pin<&mut String>
//     // projection.unpinned is &mut i32
//     *projection.unpinned += 1;
//     println!("Updated unpinned value: {}", projection.unpinned);

//     // Example 2: Self-referential struct
//     let mut self_ref =
//         SelfReferential::new(String::from("self-referential data"));

//     // Pin the struct to the stack
//     let mut pinned_self_ref = unsafe { Pin::new_unchecked(&mut self_ref) };

//     // Initialize the self-reference
//     pinned_self_ref.as_mut().init();

//     // Access the data through the self-reference
//     let ptr = pinned_self_ref.as_ref().get_pointer();
//     let value = pinned_self_ref.as_ref().get_value();

//     println!("Value: {}", value);
//     println!("Pointer points to value: {}", unsafe { &*ptr });

//     // Example 3: Enum
//     let mut enum_value =
//         PinnedEnum::Variant1(String::from("pinned enum data"), 123);
//     let mut pinned_enum = unsafe { Pin::new_unchecked(&mut enum_value) };

//     pinned_enum.as_mut().process();

//     // Example 4: Custom drop
//     let _pinned_drop = PinnedWithDrop {
//         pinned_field: 42,
//         unpinned_field: String::from("will be dropped"),
//     };

//     println!("Created PinnedWithDrop, will be dropped at end of scope");
// }

// #[test]
// fn test() {
//     main();
// }
// // [WIP finish](https://github.com/john-cd/rust_howto/issues/1120)
