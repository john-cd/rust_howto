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
    // `if let` binds the `Option` to a `Some(...)` pattern.
    // If and only if there is a value, `sprinkle_choice` is assigned the
    // content of the option.
    if let Some(sprinkle_choice) = sprinkles {
        cake.push_str(
            // `sprinkle_choice` is never "null" / unassigned.
            format!(" with a sprinkle of {}", sprinkle_choice).as_str(),
        );
    } else {
        // Optional fallback behavior if `sprinkles` is `None`.
        cake.push_str(" ready for your decorating touch!");
    }
    cake
}

fn main() {
    println!("{}", bake_cake(Some("rainbow nonpareils")));
    // The absence of a value is noted explicitly with `None`.
    println!("{}", bake_cake(None));
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
