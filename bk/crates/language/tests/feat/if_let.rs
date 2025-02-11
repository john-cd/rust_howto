// ANCHOR: example
fn main() {
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        // <-- if let
        println!("The maximum is configured to be {}", max);
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
