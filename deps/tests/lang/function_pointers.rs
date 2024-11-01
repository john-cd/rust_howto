// ANCHOR: example
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    // function pointer
    f(arg) + f(arg)
}

fn main() {
    println!("{}", do_twice(add_one, 1));
}

// ANCHOR_END: example
#[test]
fn test() {
    main();
}
