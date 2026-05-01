#![allow(dead_code)]
// ANCHOR: example
/// Struct with a type parameter `T` and a lifetime parameter `'a`.
///
/// The `T: ?Sized` trait bound indicates that `T` can be dynamically sized,
/// which will be useful in the example below.
struct DataHolder<'a, T: ?Sized> {
    data: &'a T, /* The lifetime parameter is used to specify the lifetime
                  * of the reference. */
}

/// Method implementation for the struct.
/// Note that the lifetime `'a` (declared after `impl`) is used in the struct
/// and in the method signature to ensure that the returned reference has the
/// same lifetime as the struct.
impl<'a, T> DataHolder<'a, T> {
    fn get_data(&self) -> &'a T {
        self.data
    }
}

fn main() {
    // `T` is `u8`.
    let data = DataHolder { data: &10u8 };
    println!("Data: {:?}", data.get_data());

    // The lifetime parameter is most often elided, but can be specified:
    let literal = "This string literal is of type &'static str";
    let _data: DataHolder<'static, str> = DataHolder { data: literal };
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
