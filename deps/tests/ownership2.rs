#[test]
fn test() {
    {
        let _s = String::from("hello");
    } // variable out of scope - Rust calls `drop`

    // ERROR println!("{}", s);
}
