fn main() {
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        // <-- if let
        println!("The maximum is configured to be {}", max);
    }
}

#[test]
fn test() {
    main();
}
