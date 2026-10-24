#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Applies an embedded Diesel migration and round-trips a user in SQLite.

use anyhow::Context as _;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel::table;
use diesel_migrations::EmbeddedMigrations;
use diesel_migrations::MigrationHarness;
use diesel_migrations::embed_migrations;

/// Database migrations embedded in this example binary.
const MIGRATIONS: EmbeddedMigrations =
    embed_migrations!("examples/query_builders_orms/migrations");

table! {
    users (id) {
        id -> Integer,
        name -> Text,
        email -> Text,
    }
}

/// A row stored in the `users` table.
#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
struct User {
    /// The database-generated identifier.
    id: i32,
    /// The user's display name.
    name: String,
    /// The user's email address.
    email: String,
}

/// Values inserted into the `users` table.
#[derive(Insertable)]
#[diesel(table_name = users)]
struct NewUser<'a> {
    /// The user's display name.
    name: &'a str,
    /// The user's email address.
    email: &'a str,
}

/// Applies the migration, inserts a user, and reads it back.
///
/// # Errors
///
/// Returns an error if the in-memory database cannot be opened, the migration
/// fails, or Diesel cannot insert and query the user.
fn run_example() -> anyhow::Result<User> {
    let mut connection = SqliteConnection::establish(":memory:")
        .context("open the in-memory SQLite database")?;
    connection
        .run_pending_migrations(MIGRATIONS)
        .map_err(|error| {
            anyhow::anyhow!("apply the embedded user-table migration: {error}")
        })?;

    let new_user = NewUser {
        name: "Ada Lovelace",
        email: "ada@example.com",
    };
    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(&mut connection)
        .context("insert a user")?;

    users::table
        .order(users::id.desc())
        .select(User::as_select())
        .first(&mut connection)
        .context("read the inserted user")
}

/// Runs the migrated SQLite example and prints the inserted row.
///
/// # Errors
///
/// Returns an error if migration, insertion, or query execution fails.
fn main() -> anyhow::Result<()> {
    let user = run_example()?;
    println!("User {}: {} <{}>", user.id, user.name, user.email);
    Ok(())
}
// ANCHOR_END: example

/// Runs the Diesel SQLite example.
///
/// # Errors
///
/// Returns an error if migration, insertion, or query execution fails.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Applies the embedded migration and reads back the inserted row.
    #[test]
    fn applies_migration_and_round_trips_a_user() -> anyhow::Result<()> {
        let user = run_example()?;

        assert_eq!(user.name, "Ada Lovelace");
        assert_eq!(user.email, "ada@example.com");

        Ok(())
    }
}
