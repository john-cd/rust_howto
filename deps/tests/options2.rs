fn bake_cake(sprinkles: Option<&str>) -> String {
    let mut cake = String::from("A delicious cake base...");

    // Add required ingredients

    // Handle optional sprinkles
    if let Some(sprinkle_choice) = sprinkles {
        cake.push_str(
            format!(" with a sprinkle of {}", sprinkle_choice).as_str(),
        );
    } else {
        // sprinkles is None
        cake.push_str(" ready for your decorating touch!");
    }
    cake
}

#[test]
fn test() {
    bake_cake(Some("rainbow nonpareils"));
}
