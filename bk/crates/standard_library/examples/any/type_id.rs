#![allow(dead_code)]
// ANCHOR: example
use std::any::Any;
use std::any::TypeId;

// `of` returns the `TypeId` of the generic type parameter.
//
// The `'static` bound below is necessary,
// because `TypeId` only works with `'static` types.
fn is_same_type<T: 'static, U: 'static>() -> bool {
    TypeId::of::<T>() == TypeId::of::<U>()
}

fn main() {
    // Compare types with `TypeId::of`:`
    println!("i32 vs i32: {}", is_same_type::<i32, i32>()); // true.
    println!("i32 vs u32: {}", is_same_type::<i32, u32>()); // false.

    // The `Any` trait can be used to get a `TypeId`:
    let value: &dyn Any = &"hello";
    let tid = value.type_id();
    // `TypeId` implements `Debug` but its value is just an opaque number.
    // See below to get a type name.
    println!("TypeId of value: {tid:?}");

    // However, given a smart pointer containing `dyn Any`,
    // calling `.type_id()` on the smart pointer will produce the `TypeId` of
    // the container, not the underlying object:
    let boxed: Box<dyn Any> = Box::new(3_i32);
    let boxed_id = boxed.type_id();
    // To get the `TypeId` of the inner value,
    // use `&*` to first dereference the `Box` (calling `Deref`) and then
    // immediately re-borrow.
    let actual_id = (&*boxed).type_id();

    assert_eq!(actual_id, TypeId::of::<i32>());
    assert_eq!(boxed_id, TypeId::of::<Box<dyn Any>>());

    // To get a type _name_ for diagnostics purposes, use `std::any::type_name`
    // or `type_name_of_val`. Note: the returned name may vary with compiler
    // versions.
    println!("{}", std::any::type_name::<String>());
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
