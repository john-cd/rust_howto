#![allow(dead_code)]
// ANCHOR: example
// COMING SOON
// ANCHOR_END: example
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
        pub id: u32,
        pub name: String,
        pub quantity: u32,
        pub price: f64,
    }

    /// Implementation of the Product struct.
    impl Product {
        pub fn new(id: u32, name: String, quantity: u32, price: f64) -> Self {
            Product {
                id,
                name,
                quantity,
                price,
            }
        }

        pub fn set_quantity(&mut self, quantity: u32) {
            self.quantity = quantity;
        }

        pub fn update(&mut self, name: String, price: f64) {
            self.name = name;
            self.price = price;
        }
    }
}

mod events {

    use std::sync::RwLock;

    use serde::Deserialize;
    use serde::Serialize;

    /// Events represent facts about what has happened in the system.
    ///
    /// Examples:
    /// - `ProductCreated`.
    /// - `ProductQuantityUpdated`.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    /// Represents events related to products.
    pub enum ProductEvent {
        ProductCreated {
            id: u32,
            name: String,
            quantity: u32,
            price: f64,
        },
        ProductUpdated {
            id: u32,
            name: String,
            price: f64,
        },
        ProductQuantityUpdated {
            id: u32,
            new_quantity: u32,
        },
    }

    /// `EventRepository` trait.
    ///
    /// Acts as the source of truth for the system's state.
    /// Events are stored in the Event Store.
    pub trait EventRepository {
        fn apply_event(&self, event: ProductEvent);
        fn get_events(&self, limit: Option<usize>) -> Vec<ProductEvent>;
    }

    impl EventRepository for SimpleEventStore {
        fn apply_event(&self, event: ProductEvent) {
            let mut events = self.events.write().unwrap();
            events.push(event);
        }

        // Get a snapshot of the events
        fn get_events(&self, limit: Option<usize>) -> Vec<ProductEvent> {
            let events = self.events.read().unwrap();
            if let Some(limit) = limit {
                events.iter().rev().take(limit).rev().cloned().collect()
            } else {
                events.clone()
            }
        }
    }

    // In this example, the `EventStore` is held in memory,
    // but in a real world application, it would be a database.
    pub struct SimpleEventStore {
        events: RwLock<Vec<ProductEvent>>,
    }

    impl SimpleEventStore {
        pub fn new() -> Self {
            SimpleEventStore {
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
    #[derive(Debug)]
    pub enum Command {
        CreateProduct {
            id: u32,
            name: String,
            quantity: u32,
            price: f64,
        },
        UpdateProduct {
            id: u32,
            name: String,
            price: f64,
        },
        UpdateProductQuantity {
            id: u32,
            quantity_change: i32,
        },
    }

    // Command Handler:
    // - Receives commands.
    // - Validates commands before applying them.
    // - Generates events based on successful command execution.
    pub struct CommandHandler<S: EventRepository> {
        pub event_store: S,
    }

    impl<S: EventRepository> CommandHandler<S> {
        pub fn new(event_store: S) -> Self {
            CommandHandler { event_store }
        }

        /// Validates a command before applying it.
        fn validate(&self, command: &Command) -> anyhow::Result<()> {
            match command {
                Command::CreateProduct { quantity, .. } if *quantity == 0 =>
{                     Err(anyhow::anyhow!(
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
        pub async fn handle(&self, command: Command) -> anyhow::Result<()> {
            match command {
                Command::CreateProduct {
                    id,
                    name,
                    quantity,
                    price,
                } => {
                    let event = ProductEvent::ProductCreated {
                        id,
                        name,
                        quantity,
                        price,
                    };
                    self.event_store.apply_event(event);
                    Ok(())
                }

                Command::UpdateProduct { id, name, price } => {
                    // Check if product exists by replaying events
                    if self.get_current_quantity(id).is_some() {
                        let event = ProductEvent::ProductUpdated {
                            id,
                            name,
                            price,
                        };
                        self.event_store.apply_event(event);
                        Ok(())
                    } else {
                        Err(anyhow::anyhow!("Product with id {id} not found"))
                    }
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
                                "Insufficient quantity for product {id}"
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
        pub async fn process(&self, command: Command) -> anyhow::Result<()> {
            self.validate(&command)?;
            self.handle(command).await
        }

        /// Gets the current quantity of a product by replaying events.
        fn get_current_quantity(&self, product_id: u32) -> Option<u32> {
            let events = self.event_store.get_events(None);
            let mut current_quantity: Option<u32> = None;

            for event in events.iter() {
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
                    ProductEvent::ProductUpdated { id, .. }
                        if *id == product_id =>
                    {
                        // Price/Name updates don't change quantity,
                        // but they confirm the product exists.
                        if current_quantity.is_none() {
                            current_quantity = Some(0);
                        }
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
        fn get_product(&self, id: u32) -> Option<Product>;
        // FIXME get_all_products() -> Vec<Product>;
    }

    /// `SimpleProductRepository` struct.




    /// SimpleReadStore struct.
    ///
    /// Data Access Layer (DAL) with database entities (events here).
    #[derive(Clone)]
    pub struct SimpleReadStore {
        read_model: std::sync::Arc<RwLock<HashMap<u32, Product>>>,
    }

    impl ProductRepository for SimpleReadStore {
        fn get_product(&self, id: u32) -> Option<Product> {
            let read_model = self.read_model.read().unwrap();
            read_model.get(&id).cloned()
        }
    }

    impl SimpleReadStore {
        pub fn new() -> Self {
            SimpleReadStore {
                read_model: std::sync::Arc::new(RwLock::new(HashMap::new())),
            }
        }

        fn apply_event_internal(
            read_model: &mut HashMap<u32, Product>,
            event: ProductEvent,
        ) {
            match event {
                ProductEvent::ProductCreated {
                    id,
                    name,
                    quantity,
                    price,
                } => {
                    read_model
                        .insert(id, Product::new(id, name, quantity, price));
                }
                ProductEvent::ProductUpdated { id, name, price } => {
                    if let Some(product) = read_model.get_mut(&id) {
                        product.update(name, price);
                    }
                }
                ProductEvent::ProductQuantityUpdated { id, new_quantity } => {
                    if let Some(product) = read_model.get_mut(&id) {
                        product.set_quantity(new_quantity);
                    }
                }
            }
        }

        /// Rebuilds the read model from the event store.
        ///
        /// This is used to re-create the read model from the event store.
        pub fn rebuild_read_model(&self, events: Vec<ProductEvent>) {
            let mut read_model = self.read_model.write().unwrap();
            read_model.clear();
            for event in events {
                Self::apply_event_internal(&mut read_model, event);
            }
        }
    }
}

/// Query module.
mod query {

    use super::domain::Product;
    use super::read_store::ProductRepository;

    /// Queries represent intentions to retrieve data from the system.
    #[derive(Debug)]
    pub enum Query {
        GetProduct(u32),
    }

    /// Query Handler:
    /// - Handles read requests.
    // - Maintains a read-optimized representation of the data.
    // - Is updated by processing events from the Event Store.
    // - Uses a hashmap as an in-memory database for the read model.
    pub struct QueryHandler<R: ProductRepository> {
        read_repo: R,
    }

    impl<S: ProductRepository> QueryHandler<S> {
        pub fn new(read_store: S) -> Self {
            QueryHandler {
                read_repo: read_store,
            }
        }

        /// Handles a query.
        pub async fn handle(&self, query: Query) -> anyhow::Result<Product> {
            match query {
                Query::GetProduct(id) => {
                    self.read_repo.get_product(id).ok_or_else(|| {
                        anyhow::anyhow!("Product with id {id} not found")
                    })
                }
            }
        }
    }
}

/// Main function.
use events::EventRepository;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use commands::Command;
    use query::Query;
    use std::time::Duration;
    use tokio::time;

    let event_store = events::SimpleEventStore::new();
    let command_handler = commands::CommandHandler::new(event_store);

    let read_store = read_store::SimpleReadStore::new();
    let query_handler = query::QueryHandler::new(read_store.clone());

    // Create a product
    command_handler
        .process(Command::CreateProduct {
            id: 1,
            name: "Laptop".to_string(),
            quantity: 10,
            price: 1200.0,
        })
        .await?;

    // Update product quantity
    command_handler
        .process(Command::UpdateProductQuantity {
            id: 1,
            quantity_change: -3,
        })
        .await?;

    // Rebuild read model from events
    let events = command_handler.event_store.get_events(None);
    read_store.rebuild_read_model(events);

    // Query the product
    let query = Query::GetProduct(1);
    match query_handler.handle(query).await {
        Ok(product_retrieved) => {
            println!(
                "Product returned by query via query handler: {:?}",
                product_retrieved
            );
        }
        Err(e) => {
            println!("An error occurred: {}", e);
        }
    }

    // Send an update command
    let command = Command::UpdateProduct {
        id: 1,
        name: "Laptop v2".to_string(),
        price: 1500.0,
    };
    command_handler.process(command).await?;

    // Wait for event handler to process events (simulated here by rebuild)
    time::sleep(Duration::from_millis(100)).await;
    let events = command_handler.event_store.get_events(None);
    read_store.rebuild_read_model(events);

    // Query the product again
    let query = Query::GetProduct(1);
    match query_handler.handle(query).await {
        Ok(product_retrieved) => {
            println!("Updated product: {:?}", product_retrieved);
            assert_eq!(product_retrieved.name, "Laptop v2");
            assert_eq!(product_retrieved.price, 1500.0);
        }
        Err(e) => {
            println!("An error occurred: {}", e);
        }
    }

    // Try to create another product
    command_handler
        .process(Command::CreateProduct {
            id: 2,
            name: "Mouse".to_string(),
            quantity: 20,
            price: 25.0,
        })
        .await?;

    Ok(())
}

#[tokio::test]
async fn test_cqrs() -> anyhow::Result<()> {
    // We just execute main, which shouldn't panic
    main()
}

// TODO finish
// <https://martinfowler.com/bliki/CQRS.html>
// <https://blog.cesc.cool/user-service-with-cqrs-es-example-in-rust-part-1?source=more_series_bottom_blogs>
// <https://blog.cesc.cool/user-service-with-cqrs-es-example-in-rust-part-2>
// <https://learn.microsoft.com/en-us/azure/architecture/patterns/cqrs>
// <https://doc.rust-cqrs.org/intro.html>
// <https://github.com/primait/event_sourcing.rs>
// <https://github.com/eniltrexAdmin/crappy-user>
// <https://github.com/serverlesstechnology/cqrs-demo/tree/main>
