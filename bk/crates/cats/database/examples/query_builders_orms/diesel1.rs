#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Build Diesel queries without requiring a live database connection.
//!
//! This example defines a `users` table and prints the generated SQL for an
//! insert and a filtered select.

use diesel::prelude::*;
use diesel::sqlite::Sqlite;
use diesel::table;

table! {
    users (id) {
        id -> Integer,
        name -> Text,
        email -> Text,
    }
}

/// A row of the `users` table.
#[derive(Debug, Insertable, Queryable, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(Sqlite))]
struct User {
    /// The identifier.
    id: i32,
    /// The name.
    name: String,
    /// The email address.
    email: String,
}

/// Builds insert and select queries and prints the SQL.
fn main() {
    let new_user = User {
        id: 1,
        name: "John Doe".to_owned(),
        email: "john@example.com".to_owned(),
    };

    let insert_query = diesel::insert_into(users::table).values(&new_user);
    let select_query = users::table
        .filter(users::email.like("%@example.com"))
        .select(User::as_select());

    println!(
        "INSERT SQL: {}",
        diesel::debug_query::<Sqlite, _>(&insert_query)
    );
    println!(
        "SELECT SQL: {}",
        diesel::debug_query::<Sqlite, _>(&select_query)
    );
}
// ANCHOR_END: example

/// Runs the example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
    // [finish](https://github.com/john-cd/rust_howto/issues/709)
}
