// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Create and query a model with Toasty and in-memory SQLite.
//!
//! An ORM (object-relational mapper) maps database rows to Rust structs, so you
//! create and query models instead of writing SQL by hand. An in-memory SQLite
//! database lives only while the program runs, which suits tests.

/// A user stored by the example database.
#[derive(Debug, toasty::Model)]
struct User {
    /// The database-generated identifier.
    #[key]
    #[auto]
    id: u64,
    /// The user's display name.
    name: String,
    /// The user's unique email address.
    #[unique]
    email: String,
}

/// Creates an in-memory database, inserts a user, and reads it back.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut db = toasty::Db::builder()
        .models(toasty::models!(User))
        .connect("sqlite::memory:")
        .await?;
    db.push_schema().await?;

    let inserted = toasty::create!(User {
        name: "Grace Hopper",
        email: "grace@example.com",
    })
    .exec(&mut db)
    .await?;

    let found = User::get_by_id(&mut db, &inserted.id).await?;
    assert_eq!(found.name, "Grace Hopper");
    assert_eq!(found.email, "grace@example.com");
    println!("User {}: {} <{}>", found.id, found.name, found.email);
    Ok(())
}
// ANCHOR_END: example

/// Runs the Toasty SQLite example.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies a model can be inserted and loaded without an external
    /// database.
    #[test]
    fn toasty_round_trips_a_user_with_in_memory_sqlite() -> anyhow::Result<()> {
        main()
    }
}
