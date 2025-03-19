// // ANCHOR: example
// // COMING SOON
// // ANCHOR_END: example

// use std::env;

// use diesel::prelude::*;
// use diesel::sqlite::SqliteConnection;
// use diesel::table;
// use dotenvy::dotenv;
// use serde::Deserialize;
// use serde::Serialize;

// // Define our schema
// mod schema {
//     table! {
//         users (id) {
//             id -> Integer,
//             name -> Text,
//             email -> Text,
//         }
//     }
// }

// use schema::users;
// use schema::users::dsl::users as all_users;

// #[derive(Queryable, Insertable, Serialize, Deserialize)]
// #[diesel(table_name = users)]
// struct User {
//     id: i32,
//     name: String,
//     email: String,
// }

// impl User {
//     fn new(name: String, email: String) -> Self {
//         User { id: 0, name, email }
//     }
// }

// fn establish_connection() -> SqliteConnection {
//     dotenv().ok();
//     let database_url =
//         env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//     SqliteConnection::establish(&database_url)
//         .expect(&format!("Error connecting to {}", database_url))
// }

// fn create_user(conn: &SqliteConnection, name: &str, email: &str) -> usize {
//     let new_user = User::new(name.to_string(), email.to_string());
//     diesel::insert_into(users::table)
//         .values(&new_user)
//         .execute(conn)
//         .expect("Error saving new user")
// }

// fn get_users(conn: &SqliteConnection) -> Vec<User> {
//     all_users.load::<User>(conn).expect("Error loading users")
// }

// fn main() {
//     let connection = establish_connection();

//     // Create a new user
//     create_user(&connection, "John Doe", "john@example.com");

//     // Retrieve and print all users
//     let users = get_users(&connection);
//     for user in users {
//         println!(
//             "ID: {}, Name: {}, Email: {}",
//             user.id, user.name, user.email
//         );
//     }
// }

// #[test]
// fn require_external_svc() {
//     main();
// }
// // [WIP finish SOON](https://github.com/john-cd/rust_howto/issues/709)
