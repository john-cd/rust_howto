// ANCHOR: example
use uuid::Uuid;
use uuid::uuid;

fn main() {
    // Generate a new UUID (version 4) randomly.
    let my_uuid = Uuid::new_v4();
    println!("Generated UUID: {my_uuid}");

    // Parse a UUID from a string:
    let uuid_str = "550e8400-e29b-41d4-a716-446655440000";
    match Uuid::parse_str(uuid_str) {
        Ok(parsed_uuid) => println!("Parsed UUID: {parsed_uuid}"),
        Err(e) => println!("Failed to parse UUID: {e}"),
    }
    // Use a macro to create a UUID from a string literal at compile time.
    // This is useful for defining constant UUIDs.
    const ID: Uuid = uuid!("67e55044-10b1-426f-9247-bb680e5fe0c8");
    // Print the UUID as a URN (Uniform Resource Name).
    println!("{}", ID.urn());

    // Compare UUIDs.
    // Since `another_uuid` is newly generated, it will (almost certainly) be
    // different from `my_uuid`.
    let another_uuid = Uuid::new_v4();
    if my_uuid == another_uuid {
        println!("The UUIDs are equal.");
    } else {
        println!("The UUIDs are different.");
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
