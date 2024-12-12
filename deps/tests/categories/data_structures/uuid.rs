// ANCHOR: example
use uuid::Uuid;
use uuid::uuid;

fn main() {
    // Generate a new UUID (version 4)
    let my_uuid = Uuid::new_v4();
    println!("Generated UUID: {}", my_uuid);

    // Parse a UUID from a string
    let uuid_str = "550e8400-e29b-41d4-a716-446655440000";
    match Uuid::parse_str(uuid_str) {
        Ok(parsed_uuid) => println!("Parsed UUID: {}", parsed_uuid),
        Err(e) => println!("Failed to parse UUID: {}", e),
    }
    // Use a macro
    const ID: Uuid = uuid!("67e55044-10b1-426f-9247-bb680e5fe0c8");
    // Print as a URN
    println!("{}", ID.urn());

    // Compare UUIDs
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
