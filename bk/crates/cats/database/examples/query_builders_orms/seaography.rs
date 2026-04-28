#![allow(dead_code)]
// ANCHOR: example
// Seaography is an automated GraphQL framework for SeaORM.
// You do not write query builder logic by hand. Instead, you use the
// seaography-cli to auto-generate the server from your existing database:
//
// 1. Generate entities using sea-orm-cli: sea-orm-cli generate entity -o
//    src/entities -u sqlite://sakila.db --seaography
//
// 2. Generate the GraphQL server project using seaography-cli: seaography-cli
//    ./ src/entities sqlite://sakila.db seaography-sqlite-example
//
// 3. Start the server: cd seaography-sqlite-example && cargo run
// ANCHOR_END: example

fn main() {}

#[test]
fn require_external_svc() {
    main();
}
