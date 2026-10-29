// TODO Reviewed ??-??-2026.

#![expect(
    dead_code,
    reason = "the snippet declares items to illustrate them, not to call them"
)]
// ANCHOR: example
//! CQRS (Command Query Responsibility Segregation) is an architectural
//! pattern that separates the models for reading and writing data.
//! This separation allows for independent scaling and optimization of the
//! read and write sides.
//!
//! Separation of Concerns in CQRS:
//! - Commands and queries are handled separately.
//! - The write side (commands) focuses on state changes.
//! - The read side (queries) focuses on efficient data retrieval.

/// Domain Model.
mod domain {

    /// Represents a product in the domain.
    #[derive(Debug, Clone)]
    pub struct Product {
        /// The identifier of the product.
        id: u32,
        /// The name of the product.
        name: String,
        /// The quantity in stock.
        quantity: u32,
    }

    /// Implementation of the Product struct.
    impl Product {
        /// Creates a product.
        pub fn new(id: u32, name: String, quantity: u32) -> Self {
            Self { id, name, quantity }
        }

        // Getters.
        /// Returns the identifier of the product.
        pub fn id(&self) -> u32 {
            self.id
        }

        /// Returns the name of the product.
        pub fn name(&self) -> &str {
            &self.name
        }

        /// Returns the quantity in stock.
        pub fn quantity(&self) -> u32 {
            self.quantity
        }

        /// Sets the quantity in stock.
        pub fn set_quantity(&mut self, quantity: u32) {
            self.quantity = quantity;
        }
    }
}

/// The events, which are the source of truth of the system.
mod events {

    use std::sync::RwLock;

    use serde::Deserialize;
    use serde::Serialize;

    /// Events represent facts about what has happened in the system.
    ///
    /// Examples:
    /// - `ProductCreated`.
    /// - `ProductQuantityUpdated`.
    ///
    /// Represents events related to products.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum ProductEvent {
        /// A product was created.
        ProductCreated {
            /// The identifier of the product.
            id: u32,
            /// The name of the product.
            name: String,
            /// The initial quantity.
            quantity: u32,
        },
        /// The quantity of a product was updated.
        ProductQuantityUpdated {
            /// The identifier of the product.
            id: u32,
            /// The new quantity.
            new_quantity: u32,
        },
    }

    /// `EventRepository` trait.
    ///
    /// Acts as the source of truth for the system's state.
    /// Events are stored in the Event Store.
    pub trait EventRepository {
        /// Stores an event.
        fn apply_event(&self, event: ProductEvent);

        /// Returns the events, or only the latest `limit` events if a limit is
        /// given.
        fn get_events(&self, limit: Option<usize>) -> Vec<ProductEvent>;
    }

    impl EventRepository for SimpleEventStore {
        fn apply_event(&self, event: ProductEvent) {
            let mut events = self
                .events
                .write()
                .expect("the cqrs example invariant should hold");
            events.push(event);
        }

        // Get a snapshot of the events
        fn get_events(&self, limit: Option<usize>) -> Vec<ProductEvent> {
            let events = self
                .events
                .read()
                .expect("the cqrs example invariant should hold");
            if let Some(limit) = limit {
                events.iter().rev().take(limit).rev().cloned().collect()
            } else {
                events.clone()
            }
        }
    }

    // In this example, the `EventStore` is held in memory,
    // but in a real world application, it would be a database.
    /// An event store that keeps the events in memory.
    pub struct SimpleEventStore {
        /// The events, in the order they were applied.
        events: RwLock<Vec<ProductEvent>>,
    }

    impl SimpleEventStore {
        /// Creates an empty event store.
        pub fn new() -> Self {
            Self {
                events: RwLock::new(Vec::new()),
            }
        }
    }
}

/// Commands module.
mod commands {

    use super::events::EventRepository;
    use super::events::ProductEvent;

    // Commands.
    // They represent intentions to change the system's state
    // (e.g., `CreateProduct`, `UpdateProductQuantity`).
    /// A command, which is an intention to change the state of the system.
    #[derive(Debug)]
    pub enum Command {
        /// Creates a product.
        CreateProduct {
            /// The identifier of the product.
            id: u32,
            /// The name of the product.
            name: String,
            /// The initial quantity.
            quantity: u32,
        },
        /// Changes the quantity of a product.
        UpdateProductQuantity {
            /// The identifier of the product.
            id: u32,
            /// The change in quantity, which can be negative.
            quantity_change: i32,
        },
    }

    // Command Handler:
    // - Receives commands.
    // - Validates commands before applying them.
    // - Generates events based on successful command execution.
    /// A handler that validates commands and stores the resulting events.
    pub struct CommandHandler<S: EventRepository> {
        /// The event store.
        pub event_store: S,
    }

    impl<S: EventRepository> CommandHandler<S> {
        /// Creates a handler that stores events in `event_store`.
        pub fn new(event_store: S) -> Self {
            Self { event_store }
        }

        /// Validates a command before applying it.
        fn validate(&self, command: &Command) -> anyhow::Result<()> {
            match command {
                Command::CreateProduct { quantity, .. } if *quantity == 0 => {
                    Err(anyhow::anyhow!(
                        "Cannot create product with zero quantity!"
                    ))
                }
                Command::UpdateProductQuantity {
                    quantity_change, ..
                } if *quantity_change == 0 => {
                    Err(anyhow::anyhow!("Change in quantity can't be zero!"))
                }
                _ => Ok(()),
            }
        }

        /// Handles a command and generates events.
        fn handle(&self, command: Command) -> anyhow::Result<()> {
            match command {
                Command::CreateProduct { id, name, quantity } => {
                    let event =
                        ProductEvent::ProductCreated { id, name, quantity };
                    self.event_store.apply_event(event);
                    Ok(())
                }

                Command::UpdateProductQuantity {
                    id,
                    quantity_change: quantity,
                } => {
                    let current_quantity = self.get_current_quantity(id);
                    if let Some(current) = current_quantity {
                        let new_quantity = current as i32 + quantity;
                        if new_quantity < 0 {
                            return Err(anyhow::anyhow!(
                                "Update would result in negative quantity for product {id}!"
                            ));
                        }
                        let event = ProductEvent::ProductQuantityUpdated {
                            id,
                            new_quantity: new_quantity as u32,
                        };
                        self.event_store.apply_event(event);
                        Ok(())
                    } else {
                        Err(anyhow::anyhow!("Product with id {id} not found"))
                    }
                }
            }
        }

        /// Processes a command.
        pub fn process(&self, command: Command) -> anyhow::Result<()> {
            self.validate(&command)?;
            self.handle(command)
        }

        /// Gets the current quantity of a product by replaying events.
        fn get_current_quantity(&self, product_id: u32) -> Option<u32> {
            let events = self.event_store.get_events(None);
            let mut current_quantity: Option<u32> = None;

            for event in &events {
                match event {
                    ProductEvent::ProductCreated { id, quantity, .. }
                        if *id == product_id =>
                    {
                        current_quantity = Some(*quantity);
                    }
                    ProductEvent::ProductQuantityUpdated {
                        id,
                        new_quantity,
                    } if *id == product_id => {
                        current_quantity = Some(*new_quantity);
                    }
                    _ => {}
                }
            }
            current_quantity
        }
    }
}

/// Read Store module.
mod read_store {
    use std::collections::HashMap;
    use std::sync::RwLock;

    use super::domain::Product;
    use super::events::ProductEvent;

    /// `ProductRepository` trait.
    /// Repository with domain objects.
    pub trait ProductRepository {
        /// Returns the product with the given identifier, if any.
        fn get_product(&self, id: u32) -> Option<Product>;

        /// Returns all products.
        fn get_all_products(&self) -> Vec<Product>;
    }

    /// `SimpleReadStore` struct.
    ///
    /// Data Access Layer (DAL) with database entities (events here).
    #[derive(Clone)]
    pub struct SimpleReadStore {
        /// The products, by identifier.
        read_model: std::sync::Arc<RwLock<HashMap<u32, Product>>>,
    }

    impl ProductRepository for SimpleReadStore {
        fn get_product(&self, id: u32) -> Option<Product> {
            let read_model = self
                .read_model
                .read()
                .expect("the cqrs example invariant should hold");
            read_model.get(&id).cloned()
        }

        fn get_all_products(&self) -> Vec<Product> {
            let read_model = self
                .read_model
                .read()
                .expect("the cqrs example invariant should hold");
            read_model.values().cloned().collect()
        }
    }

    impl SimpleReadStore {
        /// Creates an empty read store.
        pub fn new() -> Self {
            Self {
                read_model: std::sync::Arc::new(RwLock::new(HashMap::new())),
            }
        }

        /// Rebuilds the read model from the event store.
        ///
        /// This is used to re-create the read model from the event store.
        pub fn rebuild_read_model(&self, events: Vec<ProductEvent>) {
            let mut read_model = HashMap::new();
            for event in events {
                match event {
                    ProductEvent::ProductCreated { id, name, quantity } => {
                        read_model.insert(id, Product::new(id, name, quantity));
                    }
                    ProductEvent::ProductQuantityUpdated {
                        id,
                        new_quantity,
                    } => {
                        if let Some(product) = read_model.get_mut(&id) {
                            product.set_quantity(new_quantity);
                        }
                    }
                }
            }
            // Publish the complete snapshot with one write-lock acquisition.
            *self
                .read_model
                .write()
                .expect("the read-model lock should not be poisoned") =
                read_model;
        }
    }
}

/// Query module.
mod query {

    use super::domain::Product;
    use super::read_store::ProductRepository;

    /// Reads product data to answer queries in the CQRS example.
    ///
    /// Query Handler:
    /// - Handles read requests.
    // - Maintains a read-optimized representation of the data.
    // - Is updated by processing events from the Event Store.
    // - Uses a hashmap as an in-memory database for the read model.
    pub struct QueryHandler<R: ProductRepository> {
        /// The repository of products to read from.
        read_repo: R,
    }

    impl<S: ProductRepository> QueryHandler<S> {
        /// Creates a handler that reads from `read_store`.
        pub fn new(read_store: S) -> Self {
            Self {
                read_repo: read_store,
            }
        }

        /// Gets a product by ID.
        pub fn get_product(&self, id: u32) -> Option<Product> {
            // Business logic goes here
            self.read_repo.get_product(id)
        }

        /// Gets all products.
        pub fn get_all_products(&self) -> Vec<Product> {
            self.read_repo.get_all_products()
        }
    }
}

use events::EventRepository as _;

/// Main function.
fn main() -> anyhow::Result<()> {
    use commands::Command;

    let event_store = events::SimpleEventStore::new();
    let command_handler = commands::CommandHandler::new(event_store);

    let read_store = read_store::SimpleReadStore::new();
    let query_handler = query::QueryHandler::new(read_store.clone());

    command_handler.process(Command::CreateProduct {
        id: 1,
        name: "Laptop".to_owned(),
        quantity: 10,
    })?;

    command_handler.process(Command::UpdateProductQuantity {
        id: 1,
        quantity_change: -3,
    })?;

    let events = command_handler.event_store.get_events(None);
    read_store.rebuild_read_model(events);

    if let Some(product) = query_handler.get_product(1) {
        println!("Product: {product:?}");
    } else {
        println!("Product not found");
    }

    command_handler
        .process(Command::CreateProduct {
            id: 2,
            name: "Mouse".to_owned(),
            quantity: 20,
        })
        .expect("the cqrs example invariant should hold");

    let events = command_handler.event_store.get_events(None);
    read_store.rebuild_read_model(events);
    if let Some(product) = query_handler.get_product(2) {
        println!("Product: {product:?}");
    } else {
        println!("Product not found");
    }

    let all_products = query_handler.get_all_products();
    println!("All products: {all_products:?}");

    Ok(())
}

// ANCHOR_END: example

pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the cqrs example through its entry point.
    #[test]
    fn test_cqrs_example_entry_point() -> anyhow::Result<()> {
        // We just execute main, which shouldn't panic
        main()?;
        Ok(())
    }

    /// Verifies that rebuild replaces snapshot and applies quantity updates.
    #[test]
    fn rebuild_replaces_snapshot_and_applies_quantity_updates() {
        let read_store = read_store::SimpleReadStore::new();
        let query_handler = query::QueryHandler::new(read_store.clone());
        read_store.rebuild_read_model(vec![
            events::ProductEvent::ProductCreated {
                id: 2,
                name: "Old product".to_owned(),
                quantity: 1,
            },
        ]);
        read_store.rebuild_read_model(vec![
            events::ProductEvent::ProductCreated {
                id: 1,
                name: "Laptop".to_owned(),
                quantity: 10,
            },
            events::ProductEvent::ProductQuantityUpdated {
                id: 1,
                new_quantity: 7,
            },
        ]);

        assert!(query_handler.get_product(2).is_none());
        let product = query_handler
            .get_product(1)
            .expect("the cqrs example invariant should hold");
        assert_eq!(product.name(), "Laptop");
        assert_eq!(product.quantity(), 7);
    }

    /// Verifies that negative quantity update returns error.
    #[test]
    fn test_negative_quantity_update_returns_error() -> anyhow::Result<()> {
        let event_store = events::SimpleEventStore::new();
        let command_handler = commands::CommandHandler::new(event_store);

        command_handler.process(commands::Command::CreateProduct {
            id: 1,
            name: "Test Product".to_owned(),
            quantity: 10,
        })?;

        let result =
            command_handler.process(commands::Command::UpdateProductQuantity {
                id: 1,
                quantity_change: -11,
            });

        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Update would result in negative quantity")
        );

        Ok(())
    }

    /// Verifies that get all products.
    #[test]
    fn test_get_all_products() {
        let read_store = read_store::SimpleReadStore::new();
        let query_handler = query::QueryHandler::new(read_store.clone());

        assert!(query_handler.get_all_products().is_empty());

        read_store.rebuild_read_model(vec![
            events::ProductEvent::ProductCreated {
                id: 1,
                name: "Laptop".to_owned(),
                quantity: 10,
            },
            events::ProductEvent::ProductCreated {
                id: 2,
                name: "Mouse".to_owned(),
                quantity: 20,
            },
        ]);

        let products = query_handler.get_all_products();
        assert_eq!(products.len(), 2);
        assert!(products.iter().any(|p| p.id() == 1
            && p.name() == "Laptop"
            && p.quantity() == 10));
        assert!(
            products.iter().any(|p| p.id() == 2
                && p.name() == "Mouse"
                && p.quantity() == 20)
        );
    }
}
// Deferred (#1748): event store snapshots and read-model reconstruction need a scope decision
// before any code is added.
// Notes:
// - This example can be extended with event store snapshots and more advanced
//   read-model reconstruction.
// - See the CQRS pattern for additional architecture guidance: https://martinfowler.com/bliki/CQRS.html
//   https://blog.cesc.cool/user-service-with-cqrs-es-example-in-rust-part-1 https://blog.cesc.cool/user-service-with-cqrs-es-example-in-rust-part-2
//   https://learn.microsoft.com/en-us/azure/architecture/patterns/cqrs https://doc.rust-cqrs.org/intro.html
