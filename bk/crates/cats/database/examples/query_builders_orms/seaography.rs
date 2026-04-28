#![allow(dead_code)]
// ANCHOR: example
// Seaography is an automated GraphQL framework for SeaORM.
// It is primarily driven by generated code rather than hand-written query
// builders.
//
// 1. Generate SeaORM entities from your existing database:
// ```sh
// sea-orm-cli generate entity -o src/entities -u sqlite://sakila.db --seaography
// ```
//
// 2. Generate the GraphQL server project:
// ```sh
// seaography-cli ./ src/entities sqlite://sakila.db seaography-sqlite-example
// ```
//
// 3. Run the generated server:
// ```sh
// cd seaography-sqlite-example && cargo run
// ```
// ANCHOR_END: example

fn main() {} // TODO: Add an example of using the generated code from Seaography to interact with the database.

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn require_external_svc() {
        main();
    }
}
