// ANCHOR: example
trait Test<T> {
    fn test(_t: T);
}

struct SomeStruct;

// Note the <> in two places:
impl<T> Test<T> for SomeStruct {
    fn test(_t: T) {
        println!("test");
    }
}

fn main() {
    SomeStruct::test(1);
    SomeStruct::test(true);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
