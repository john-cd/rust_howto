fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    // function pointer
    f(arg) + f(arg)
}

#[test]
fn test() {
    println!("{}", do_twice(add_one, 1));
}
