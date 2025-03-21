// #![allow(dead_code)]
// // ANCHOR: example
// // COMING SOON
// // ANCHOR_END: example
// use anyhow::Result;

// // Separation of Concerns in CQRS:
// // - Commands and queries are handled separately.
// // - The write side (commands) focuses on state changes.
// // - The read side (queries) focuses on efficient data retrieval.

// // Domain Model
// mod domain {

//     #[derive(Debug, Clone)]
//     pub struct Product {
//         id: u32,
//         name: String,
//         quantity: u32,
//     }

//     impl Product {
//         pub fn new(id: u32, name: String, quantity: u32) -> Self {
//             Product { id, name, quantity }
//         }

//         // Getters
//         pub fn id(&self) -> u32 {
//             self.id
//         }

//         pub fn name(&self) -> &str {
//             &self.name
//         }

//         pub fn quantity(&self) -> u32 {
//             self.quantity
//         }

//         pub fn set_quantity(&mut self, quantity: u32) {
//             self.quantity = quantity;
//         }
//     }
// }

// mod events {

//     use serde::Deserialize;
//     use serde::Serialize;
//     use std::sync::RwLock;

//     // Events
//     // - Represent facts about what has happened in the system
//     // (e.g., ProductCreated, ProductQuantityUpdated).
//     // - Are stored in the Event Store.
//     #[derive(Debug, Clone, Serialize, Deserialize)]
//     pub enum ProductEvent {
//         ProductCreated {
//             id: u32,
//             name: String,
//             quantity: u32,
//         },
//         ProductQuantityUpdated {
//             id: u32,
//             new_quantity: u32,
//         },
//     }

//     // Optional: track event name and version
//     // pub trait Event {
//     //     fn event_type(&self) -> String;
//     //     fn event_version(&self) -> String {
//     //         "1.0".to_string()
//     //     }
//     // }

//     // impl Event for ProductEvent {
//     //     fn event_type(&self) -> String {
//     //         let event_type: &str = match self {
//     //             ProductEvent::ProductCreated { .. } => "ProductCreated",
//     //             ProductEvent::ProductQuantityUpdated { .. } => {
//     //                 "ProductQuantityUpdated"
//     //             }
//     //         };
//     //         event_type.to_string()
//     //     }
//     // }

//     // Acts as the source of truth for the system's state.
//     pub trait EventRepository {
//         fn apply_event(&self, event: ProductEvent);
//         fn get_events(&self) -> Vec<ProductEvent>;
//     }

//     impl EventRepository for SimpleEventStore {
//         fn apply_event(&self, event: ProductEvent) {
//             let mut events = self.events.write().unwrap();
//             events.push(event);
//         }

//         // Get a snapshot of the events
//         fn get_events(&self) -> Vec<ProductEvent> {
//             let events = self.events.read().unwrap();
//             events.clone() // FIXME
//         }
//         // FIXME get most recent useful events by reverse iteration
//     }

//     // In this example, the EventStore is held in memory,
//     // but in a real world application, it would be a database.
//     pub struct SimpleEventStore {
//         events: RwLock<Vec<ProductEvent>>,
//     }

//     impl SimpleEventStore {
//         pub fn new() -> Self {
//             SimpleEventStore {
//                 events: RwLock::new(Vec::new()),
//             }
//         }
//     }
// }

// mod commands {

//     use super::events::EventRepository;
//     use super::events::ProductEvent;
//     use anyhow::Result;

//     // Commands
//     // Represent intentions to change the system's state
//     // (e.g., CreateProduct, UpdateProductQuantity).
//     #[derive(Debug)]
//     pub enum Command {
//         CreateProduct {
//             id: u32,
//             name: String,
//             quantity: u32,
//         },
//         UpdateProductQuantity {
//             id: u32,
//             quantity_change: i32,
//         },
//     }

//     // Command Handler:
//     // - Receives commands.
//     // - Validates commands before applying them.
//     // - Generates events based on successful command execution.
//     pub struct CommandHandler<S: EventRepository> {
//         event_store: S,
//     }

//     impl<S: EventRepository> CommandHandler<S> {
//         pub fn new(event_store: S) -> Self {
//             CommandHandler { event_store }
//         }

//         fn validate(&self, command: &Command) -> Result<()> {
//             match command {
//                 Command::CreateProduct { quantity, .. } if *quantity == 0 =>
// {                     Err(anyhow::anyhow!(
//                         "Cannot create product with zero quantity!"
//                     ))
//                 }
//                 Command::UpdateProductQuantity {
//                     quantity_change, ..
//                 } if *quantity_change == 0 => {
//                     Err(anyhow::anyhow!("Change in quantity can't be zero!"))
//                 }
//                 _ => Ok(()),
//             }
//         }

//         fn handle(&self, command: Command) -> Result<()> {
//             match command {
//                 Command::CreateProduct { id, name, quantity } => {
//                     let event =
//                         ProductEvent::ProductCreated { id, name, quantity };
//                     self.event_store.apply_event(event);
//                     Ok(())
//                 }

//                 Command::UpdateProductQuantity {
//                     id,
//                     quantity_change: quantity,
//                 } => {
//                     let current_quantity = self.get_current_quantity(id);
//                     if let Some(current) = current_quantity {
//                         let new_quantity =
//                             (current as i32 + quantity).max(0) as u32; //
// FIXME handle errors                         let event =
// ProductEvent::ProductQuantityUpdated {                             id,
//                             new_quantity,
//                         };
//                         self.event_store.apply_event(event);
//                         Ok(())
//                     } else {
//                         Err(anyhow::anyhow!("Product with id {} not found",
// id))                     }
//                 }
//             }
//         }

//         pub fn process(&self, command: Command) -> Result<()> {
//             self.validate(&command)?;
//             self.handle(command)
//         }

//         fn get_current_quantity(&self, product_id: u32) -> Option<u32> {
//             let events = self.event_store.get_events();
//             let mut current_quantity: Option<u32> = None;

//             for event in events.iter() {
//                 match event {
//                     ProductEvent::ProductCreated { id, quantity, .. }
//                         if *id == product_id =>
//                     {
//                         current_quantity = Some(*quantity);
//                     }
//                     ProductEvent::ProductQuantityUpdated {
//                         id,
//                         new_quantity,
//                     } if *id == product_id => {
//                         current_quantity = Some(*new_quantity);
//                     }
//                     _ => {}
//                 }
//             }
//             current_quantity
//         }
//     }
// }

// mod read_store {
//     use std::collections::HashMap;
//     use std::sync::RwLock;

//     use super::domain::Product;
//     use super::events::ProductEvent;

//     // FIXME repository with domain objects
//     pub trait ProductRepository {
//         fn get_product(&self, id: u32) -> Option<Product>;
//         // FIXME get_all_products() -> Vec<Product>;
//     }

//     pub struct SimpleProductRepository;

//     impl ProductRepository for SimpleProductRepository {
//         fn get_product(&self, id: u32) -> Option<Product> {
//         }
//         // FIXME consume events
//     }

//     // FIXME DAL with database entities (events here)
//     pub struct SimpleReadStore {
//         read_model: RwLock<HashMap<u32, Product>>,
//     }

//     impl SimpleReadStore {
//         pub fn new() -> Self {
//             SimpleReadStore {
//                 read_model: RwLock::new(HashMap::new()),
//             }
//         }

//         // FIXME split into store and repo portions?
//         fn update(&self, event: ProductEvent) {
//             match event {
//                 ProductEvent::ProductCreated { id, name, quantity } => {
//                     let mut read_model = self.read_model.write().unwrap();
//                     read_model.insert(id, Product::new(id, name, quantity));
//                 }
//                 ProductEvent::ProductQuantityUpdated { id, new_quantity } =>
// {                     let mut read_model = self.read_model.write().unwrap();
//                     if let Some(product) = read_model.get_mut(&id) {
//                         product.set_quantity(new_quantity);
//                     }
//                 }
//             }
//         }

//         // rebuild_read_model() is used to re-create the read model
//         // from the event store.
//         fn rebuild_read_model(&self, events: Vec<ProductEvent>) {
//             let mut read_model = self.read_model.write().unwrap();
//             read_model.clear();
//             for event in events.into_iter() {
//                 self.update(event);
//             }
//         }
//     }
// }

// mod query {

//     use super::domain::Product;
//     use super::read_store::ProductRepository;

//     // Query Handler
//     // - Handles read requests.
//     // - Maintains a read-optimized representation of the data.
//     // - Is updated by processing events from the Event Store.
//     // - Uses a hashmap as an in-memory database for the read model.
//     pub struct QueryHandler<R: ProductRepository> {
//         read_repo: R,
//     }

//     impl<S: ProductRepository> QueryHandler<S> {
//         pub fn new(read_store: S) -> Self {
//             QueryHandler {
//                 read_repo: read_store,
//             }
//         }

//         fn get_product(&self, id: u32) -> Option<Product> {
//             // Business logic goes here
//             self.read_repo.get_product(id)
//         }
//     }
// }

// fn main() -> anyhow::Result<()> {
//     use commands::Command;

//     let event_store = events::SimpleEventStore::new();
//     let command_handler = commands::CommandHandler::new(event_store);

//     let read_store = read_store::SimpleReadStore::new();
//     let query_handler = query::QueryHandler::new(read_store);

//     command_handler.process(Command::CreateProduct {
//         id: 1,
//         name: "Laptop".to_string(),
//         quantity: 10,
//     })?;

//     command_handler.process(Command::UpdateProductQuantity {
//         id: 1,
//         quantity_change: -3,
//     })?;

//     //query_handler.rebuild_read_model();

//     if let Some(product) = query_handler.get_product(1) {
//         println!("Product: {:?}", product);
//     } else {
//         println!("Product not found");
//     }

//     command_handler
//         .process(Command::CreateProduct {
//             id: 2,
//             name: "Mouse".to_string(),
//             quantity: 20,
//         })
//         .unwrap();

//     // query_handler.rebuild_read_model();
//     // if let Some(product) = query_handler.get_product(2) {
//     //     println!("Product: {:?}", product);
//     // } else {
//     //     println!("Product not found");
//     // }
//     Ok(())
// }

// #[test]
// fn test() -> Result<()> {
//     main()?;
//     Ok(())
// }

// FIXME finish NOW

// // https://martinfowler.com/bliki/CQRS.html
// // https://blog.cesc.cool/user-service-with-cqrs-es-example-in-rust-part-1?source=more_series_bottom_blogs
// // https://blog.cesc.cool/user-service-with-cqrs-es-example-in-rust-part-2
// // https://learn.microsoft.com/en-us/azure/architecture/patterns/cqrs
// // https://doc.rust-cqrs.org/intro.html
// // https://github.com/primait/event_sourcing.rs
// // https://github.com/eniltrexAdmin/crappy-user
// // https://github.com/serverlesstechnology/cqrs-demo/tree/main
