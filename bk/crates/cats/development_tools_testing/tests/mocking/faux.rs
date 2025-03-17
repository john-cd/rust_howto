// // ANCHOR: example
// // COMING SOON
// // ANCHOR_END: example
// use faux::create;
// use faux::when;

// // The trait we want to mock
// trait Database {
//     fn get_user(&self, id: u64) -> Option<User>;
//     fn save_user(&self, user: &User) -> bool;
// }

// // A struct using the Database
// struct UserService<D: Database> {
//     database: D,
// }

// #[derive(Debug, Clone, PartialEq)]
// struct User {
//     id: u64,
//     name: String,
// }

// // Implementation using the trait
// impl<D: Database> UserService<D> {
//     fn new(database: D) -> Self {
//         Self { database }
//     }

//     fn get_user_name(&self, id: u64) -> Option<String> {
//         self.database.get_user(id).map(|user| user.name)
//     }

//     fn update_user_name(&self, id: u64, name: String) -> bool {
//         if let Some(mut user) = self.database.get_user(id) {
//             user.name = name;
//             self.database.save_user(&user)
//         } else {
//             false
//         }
//     }
// }

// // Create a mockable implementation
// #[faux::create]
// struct MockDatabase {}

// // Implement the trait for our mock
// #[faux::methods]
// impl Database for MockDatabase {
//     fn get_user(&self, id: u64) -> Option<User> {
//         // This will be mocked
//         unimplemented!()
//     }

//     fn save_user(&self, user: &User) -> bool {
//         // This will be mocked
//         unimplemented!()
//     }
// }

// // Test using the mock
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_get_user_name() {
//         // Create a mock
//         let mut mock_db = MockDatabase::faux();

//         // Define behavior
//         when!(mock_db.get_user).then(|id| {
//             if id == 1 {
//                 Some(User {
//                     id: 1,
//                     name: "Alice".to_string(),
//                 })
//             } else {
//                 None
//             }
//         });

//         // Create service with mock
//         let service = UserService::new(mock_db);

//         // Test the service
//         assert_eq!(service.get_user_name(1), Some("Alice".to_string()));
//         assert_eq!(service.get_user_name(2), None);
//     }

//     #[test]
//     fn test_update_user_name() {
//         // Create a mock
//         let mut mock_db = MockDatabase::faux();
//         let user = User {
//             id: 1,
//             name: "Alice".to_string(),
//         };

//         // Configure get_user to return our test user
//         when!(mock_db.get_user)
//             .then(|id| if id == 1 { Some(user.clone()) } else { None });

//         // Configure save_user to succeed
//         when!(mock_db.save_user).then(|_| true);

//         // Create service with mock
//         let service = UserService::new(mock_db);

//         // Test update succeeds
//         assert!(service.update_user_name(1, "Bob".to_string()));
//         // Test update fails for non-existent user
//         assert!(!service.update_user_name(2, "Charlie".to_string()));
//     }
// }

// #[test]
// #[ignore = "not yet implemented"]
// fn test() {
//     main();
// }
// // [add example](https://github.com/john-cd/rust_howto/issues/1126)
