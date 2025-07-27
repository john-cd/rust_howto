#![allow(dead_code)]
// ANCHOR: example
use std::any::Any;
use std::any::TypeId;

// The `'static` bound is necessary, because `TypeId` only works with `'static` types.
fn is_same_type<T: 'static, U: 'static>() -> bool {
    // `of` returns the `TypeId` of the generic type parameter.
    TypeId::of::<T>() == TypeId::of::<U>()
}

fn main() {
    // Compare types with `TypeId::of`:`
    println!("i32 vs i32: {}", is_same_type::<i32, i32>()); // true.
    println!("i32 vs u32: {}", is_same_type::<i32, u32>()); // false.

    // `Any` can be used to get a `TypeId`:
    let value: &dyn Any = &"hello";
    let tid = value.type_id();
    // `TypeId` implements `Debug`:
    println!("TypeId of value: {tid:?}");

    // Given a smart pointer containing `dyn Any`,
    let boxed: Box<dyn Any> = Box::new(3_i32);
    // Calling `.type_id()` on the smart pointer will produce the `TypeId` of the container, not the underlying object.
    let boxed_id = boxed.type_id();
    // To get the `TypeId` of the inner value,
    // use `&*` to first dereference the `Box` (calling `Deref`) and then immediately re-borrow.
    let actual_id = (&*boxed).type_id();

    assert_eq!(actual_id, TypeId::of::<i32>());
    assert_eq!(boxed_id, TypeId::of::<Box<dyn Any>>());
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
