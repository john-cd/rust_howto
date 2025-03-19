// // ANCHOR: example
// // COMING SOON
// // ANCHOR_END: example
// use salsa::Database;

// // This example demonstrates a simple dependency graph:
// // derived_string depends on computed_length, which depends on input_string.
// // When you change input_string, Salsa automatically recomputes
// computed_length and derived_string. // If you change it back to the original
// value, Salsa avoids recomputation.

// // Defines the interface for your queries. All queries must be associated
// with a query group. #[salsa::query_group(crate::MyDatabase)]
// trait MyQueries: salsa::Database {
//     // Query functions. They take &dyn MyQueries as an argument, allowing
// them to access other queries in the group.     fn input_string(&self) ->
// String;     fn computed_length(&self) -> usize;
//     fn derived_string(&self) -> String;
// }

// fn input_string(db: &dyn MyQueries) -> String {
//     // This is the input, directly provided by the user.
//     // In a real application, this might come from a file,
//     // command-line arguments, etc.
//     "Hello, Salsa!".to_string()
// }

// fn computed_length(db: &dyn MyQueries) -> usize {
//     let input = db.input_string();
//     input.len()
// }

// fn derived_string(db: &dyn MyQueries) -> String {
//     let input = db.input_string();
//     let length = db.computed_length();
//     format!("{}: {}", input, length)
// }

// // This struct holds the actual data for your queries and implements the
// MyQueries trait #[derive(Debug, Clone, PartialEq, Eq)]
// struct MyState {
//     input: String,
// }

// impl MyQueries for MyState {
//     fn input_string(&self) -> String {
//         self.input.clone()
//     }

//     fn computed_length(&self) -> usize {
//         computed_length(self)
//     }

//     fn derived_string(&self) -> String {
//         derived_string(self)
//     }
// }

// // Tells Salsa that MyState is a database.
// impl salsa::Database for MyState {}

// fn main() {
//     let mut state = MyState {
//         input: "Initial Value".to_string(),
//     };

//     let db = &mut state;  // Important: db needs to be mutable for set()

//     // Initial computation
//     println!("Initial string: {}", db.input_string());
//     println!("Initial length: {}", db.computed_length());
//     println!("Initial derived: {}", db.derived_string());

//     // Change the input
//     state.input = "A new string!".to_string();
//     db.set_input_string("A new string!".to_string()); // Update the input in
// Salsa

//     // Recompute, Salsa will efficiently update the affected values
//     println!("New string: {}", db.input_string());
//     println!("New length: {}", db.computed_length());
//     println!("New derived: {}", db.derived_string());

//     // Demonstrate memoization: changing the input back to the original
//     // value should not cause recomputation.
//     state.input = "Initial Value".to_string();
//     db.set_input_string("Initial Value".to_string()); // Update the input in
// Salsa

//     println!("String (again): {}", db.input_string()); // Should be fast
//     println!("Length (again): {}", db.computed_length()); // Should be fast
//     println!("Derived (again): {}", db.derived_string()); // Should be fast

// }

// // Required for the query_group macro
// mod __salsa_internal {
//     pub use salsa;
// }

// #[test]
// fn test() -> anyhow::Result<()> {
//     main()?;
//     Ok(())
// }
// // TODO WIP finish
