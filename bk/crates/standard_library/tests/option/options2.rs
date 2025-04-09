// ANCHOR: example
/// This function simulates baking a cake.
///
/// It takes an optional `sprinkles` argument, which determines
/// whether or not to add sprinkles to the cake.
///
/// Returns a string describing the cake.
fn bake_cake(sprinkles: Option<&str>) -> String {
    let mut cake = String::from("A delicious cake...");

    // Add required ingredients.

    // Handle optional sprinkles.
    if let Some(sprinkle_choice) = sprinkles {
        cake.push_str(
            format!(" with a sprinkle of {}", sprinkle_choice).as_str(),
        );
    } else {
        // sprinkles is `None`.
        cake.push_str(" ready for your decorating touch!");
    }
    cake
}

fn main() {
    print!("{}", bake_cake(Some("rainbow nonpareils")));
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
