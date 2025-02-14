// ANCHOR: example
use std::cell::Cell;

struct MyStruct<T> {
    // The Cell type allows for interior mutability.
    value: Cell<T>,
}

impl<T> MyStruct<T> {
    fn new(value: T) -> Self {
        MyStruct {
            value: Cell::new(value),
        }
    }

    // Replace the interior value, dropping the replaced value.
    fn set_value(&self, new_value: T) {
        self.value.set(new_value);
    }
}

impl<T: Copy> MyStruct<T> {
    fn get_value(&self) -> T {
        // For types that implement `Copy`, the `get` method
        // retrieves the current interior value by duplicating it.
        self.value.get()
    }
}

// For types that implement `Default`, the take method replaces the current
// interior value with `Default::default()` and returns the replaced value.
impl<T: Default> MyStruct<T> {
    fn take_value(&self) -> T {
        self.value.take()
    }
}

fn main() {
    let my_int_struct = MyStruct::new(42);
    let my_int_ref = &my_int_struct;

    // Get the current value (i32 is Copy)
    println!("Initial value: {}", my_int_struct.get_value());

    // Set a new value, while using an immutable reference to the struct
    my_int_ref.set_value(100);

    // Get the updated value
    println!("Updated value: {}", my_int_ref.get_value());

    let my_string_struct = MyStruct::new("example".to_string());
    let my_string_ref = &my_string_struct;

    // ERROR, `String` is not `Copy`: my_string.get_value();

    // Get the value, this time using the `take_value` method
    println!("String: {}", my_string_ref.take_value());

    assert_eq!(my_string_ref.take_value(), "");

    // Replace the current interior value and returns the replaced value.
    assert_eq!(my_string_struct.value.replace("example2".into()), "");

    // `into_inner` consumes the Cell<T> and returns the interior value.
    let _inner = my_string_struct.value.into_inner();
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
