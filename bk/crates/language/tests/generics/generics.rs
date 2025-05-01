// ANCHOR: example
use std::collections::HashMap;
use std::fmt::Debug;

/// Function with a type parameter and a lifetime.
#[allow(clippy::needless_lifetimes)]
fn print_data<'a, T: std::fmt::Debug>(value: &'a T) {
    println!("Data: {:?}", value);
}

/// Type alias with a type parameter.
type StringMap<K> = HashMap<K, String>;

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

/// Trait with a type parameter.
trait Processor<T> {
    fn process(&self, item: T) -> String;
}

/// Tuple struct with a type parameter.
struct TupleStruct<T>(T);

// Implementing the trait for the tuple struct.
impl<T: Debug> Processor<T> for TupleStruct<T> {
    fn process(&self, item: T) -> String {
        format!("Processed: {:?}", item)
    }
}

// Const generic parameters allow items to be generic over constant values.
struct InnerArray<T, const N: usize>([T; N]);

fn main() {
    print_data(&10);

    let _map = StringMap::<String>::new();

    let data = DataHolder { data: &10 };
    println!("Data: {:?}", data.get_data());

    let tuple = TupleStruct(10);
    println!("{}", tuple.process(10));

    let _inner_array: InnerArray<i32, 3> = InnerArray([1, 2, 3]);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
