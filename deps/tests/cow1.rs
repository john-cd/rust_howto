use std::borrow::Borrow;

#[test]
fn test() {
    let mut my_string = String::new();
    let example = std::borrow::Cow::from("example");
    my_string.push_str(example.borrow());
    println!("{}", my_string);
}
