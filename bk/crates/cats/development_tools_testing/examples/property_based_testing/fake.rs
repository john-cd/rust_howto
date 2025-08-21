#![allow(dead_code)]
// ANCHOR: example
// COMING SOON
// ANCHOR_END: example
//! This example demonstrates how to use the `fake` crate to generate fake data
//! for testing. It covers basic usage, generating data for custom structs, and
//! using different data providers.
//!
//! Add to your `Cargo.toml`:
//! ```toml
//! fake = { version = "4.3.0", features = [ "derive" ] }
//! ```

// Trait that generates fake values given a type that implements `Dummy`.
use fake::Dummy;
use fake::Fake;
// Generates default fake value for given type using `Fake`.
use fake::Faker;
// Fake value generation for specific formats:
use fake::faker::address::en::CityName;
use fake::faker::address::en::StreetName;
use fake::faker::address::en::ZipCode;
use fake::faker::company::en::CompanyName;
use fake::faker::internet::en::SafeEmail;
use fake::faker::internet::en::UserAgent;
use fake::faker::lorem::en::Sentence;
use fake::faker::name::en::FirstName;
use fake::faker::name::en::LastName;

// A structure to represent a User.
#[derive(Debug)]
struct User {
    id: u64,
    first_name: String,
    last_name: String,
    email: String,
    age: u8,
    active: bool,
}

// The `Fake` trait is implemented for any type that implements `Dummy`:
// Dummy should be implemented instead, and you get the `Fake` implementation
// for free.

// A structure to represent an Order.
#[derive(Debug, Dummy)]
struct Order {
    #[dummy(faker = "1000..2000")]
    id: u64,
    user_id: u64,
    items: Vec<String>,
    total: f64,
    shipping_address: Address,
    created_at: String,
    // #[dummy(default)]
    // other: String,
}

#[derive(Debug, Dummy)]
struct Address {
    street: String,
    city: String,
    zip: String,
}

fn main() {
    // Use `Faker` to generate default fake value of given type.
    let tuple = Faker.fake::<(u8, u32, f32)>();
    println!("Tuple: {tuple:?}");
    println!("String: {}", Faker.fake::<String>());
    assert_eq!(10.fake::<String>().len(), 10);
    let a: [[u8; 2]; 3] = (1..10).fake();
    println!("{a:?}");
    let b: Option<usize> = (1..10).fake();
    println!("{b:?}");

    // Generate a random user.
    let user = User {
        id: Faker.fake(),
        first_name: FirstName().fake(),
        last_name: LastName().fake(),
        email: SafeEmail().fake(),
        age: (18..65).fake(),
        active: Faker.fake(),
    };
    println!("Generated User: {user:#?}");

    // Generate multiple random users.
    let users: Vec<User> = (0..5)
        .map(|_| User {
            id: Faker.fake(),
            first_name: FirstName().fake(),
            last_name: LastName().fake(),
            email: SafeEmail().fake(),
            age: (18..65).fake(),
            active: Faker.fake(),
        })
        .collect();
    println!("Generated Users: {users:#?}");

    // Generate a random order with random items.

    // type derived Dummy
    // let f: Foo = Faker.fake();
    // println!("{f:?}");
    let order = Order {
        id: Faker.fake(),
        user_id: user.id,
        items: (0..3).map(|_| CompanyName().fake()).collect(), // FIXME
        total: (50.0..500.0).fake(),
        shipping_address: Address {
            street: StreetName().fake(),
            city: CityName().fake(),
            zip: ZipCode().fake(),
        },
        created_at: format!(
            "2024-{:02}-{:02}",
            (1..12).fake::<u8>(),
            (1..28).fake::<u8>()
        ),
    };
    println!("Generated Order: {order:#?}");

    // Generate random sentences:
    let sentences: Vec<String> =
        (0..3).map(|_| Sentence(3..10).fake()).collect();
    println!("Random Sentences: {sentences:#?}");

    // Generate random user agents:
    let user_agents: Vec<String> = (0..3).map(|_| UserAgent().fake()).collect();
    println!("User Agents: {user_agents:#?}");
}

#[test]
fn test() {
    main();
}
// [review https://docs.rs/fake/4.2.0/fake/index.html](https://github.com/john-cd/rust_howto/issues/1124)
