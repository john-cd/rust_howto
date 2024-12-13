// ANCHOR: example
use derive_more::Add;
use derive_more::Display;
use derive_more::From;
use derive_more::Into;

#[derive(PartialEq, From, Add)]
struct MyInt(i32);

#[derive(PartialEq, From, Into)]
struct Point2D {
    x: i32,
    y: i32,
}

#[derive(PartialEq, From, Add, Display)]
enum MyEnum {
    #[display("int: {}", _0)]
    Int(i32),
    Uint(u32),
    #[display("nothing")]
    Nothing,
}

fn main() {
    assert!(MyInt(11) == MyInt(5) + 6.into());
    assert!((5, 6) == Point2D { x: 5, y: 6 }.into());
    assert!(MyEnum::Int(15) == (MyEnum::Int(8) + 7.into()).unwrap());
    assert!(MyEnum::Int(15).to_string() == "int: 15");
    assert!(MyEnum::Uint(42).to_string() == "42");
    assert!(MyEnum::Nothing.to_string() == "nothing");
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
// TODO P1 println!("{}", );
