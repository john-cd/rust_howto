// ANCHOR: example
fn main() {
    let mut robot_name = Some(String::from("Bender"));

    match robot_name {
        Some(ref name) => println!("Name (immutable ref): {}", name), /* `name` is `&String` */
        None => (),
    }

    match robot_name {
        Some(ref mut name_mut) => name_mut.push_str(" Rodriguez"), /* `name_mut` is `&mut String` */
        None => (),
    }
    println!("Full name: {:?}", robot_name); // Some("Bender Rodriguez")
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
