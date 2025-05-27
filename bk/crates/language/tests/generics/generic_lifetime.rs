// ANCHOR: example
/// Struct with a type parameter and a lifetime.
struct DataHolder<'a, T> {
    data: &'a T,
}

/// Method implementation for the struct.
impl<'a, T> DataHolder<'a, T> {
    fn get_data(&self) -> &'a T {
        self.data
    }
}

fn main() {
    // `T` is `u8`.
    let data = DataHolder { data: &10u8 };
    println!("Data: {:?}", data.get_data());
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
