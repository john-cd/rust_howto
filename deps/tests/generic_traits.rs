trait Test<T> {
    fn test(_t: T);
}

struct SomeStruct;

impl<T> Test<T> for SomeStruct {
    // note the <> in two places
    fn test(_t: T) {
        println!("test");
    }
}

#[test]
fn test() {
    SomeStruct::test(1);
    SomeStruct::test(true);
}
