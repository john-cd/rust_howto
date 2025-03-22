#![allow(dead_code)]
// ANCHOR: example
// COMING SOON
// ANCHOR_END: example
// Trait that generates fake values given a type that implements `Dummy`.
use fake::Fake;
// Generate default fake value for given type using `Fake`.
use fake::Faker;
// Fake value generation for specific formats.
use fake::faker::address::en::CityName;
use fake::faker::address::en::StreetName;
use fake::faker::address::en::ZipCode;
use fake::faker::company::en::CompanyName;
use fake::faker::internet::en::SafeEmail;
use fake::faker::internet::en::UserAgent;
use fake::faker::lorem::en::Sentence;
use fake::faker::name::en::FirstName;
use fake::faker::name::en::LastName;

// A structure to represent a User
#[derive(Debug)]
struct User {
    id: u64,
    first_name: String,
    last_name: String,
    email: String,
    age: u8,
    active: bool,
}

// A structure to represent an Order
#[derive(Debug)]
struct Order {
    id: u64,
    user_id: u64,
    items: Vec<String>,
    total: f64,
    shipping_address: Address,
    created_at: String,
}

#[derive(Debug)]
struct Address {
    street: String,
    city: String,
    zip: String,
}

fn main() {
    // Use `Faker` to generate default fake value of given type
    let tuple = Faker.fake::<(u8, u32, f32)>();
    println!("Tuple: {:?}", tuple);
    println!("String: {}", Faker.fake::<String>());

    // Generate a random user
    let user = User {
        id: Faker.fake(),
        first_name: FirstName().fake(),
        last_name: LastName().fake(),
        email: SafeEmail().fake(),
        age: (18..65).fake(),
        active: Faker.fake(),
    };
    println!("Generated User: {:#?}", user);

    // Generate multiple random users
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
    println!("Generated Users: {:#?}", users);

    // Generate a random order with random items
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
    println!("Generated Order: {:#?}", order);

    // Generate random sentences
    let sentences: Vec<String> =
        (0..3).map(|_| Sentence(3..10).fake()).collect();
    println!("Random Sentences: {:#?}", sentences);

    // Generate random user agents
    let user_agents: Vec<String> = (0..3).map(|_| UserAgent().fake()).collect();
    println!("User Agents: {:#?}", user_agents);
}

#[test]
fn test() {
    main();
}
// [review https://docs.rs/fake/4.2.0/fake/index.html cover derive, Dummy, complex types... NOW](https://github.com/john-cd/rust_howto/issues/1124)
