// ANCHOR: example
use sqlx::SqlitePool;

// In Cargo.toml, add the following dependencies:
// sqlx = { version = "0.8", features = [ "runtime-tokio", "sqlite" ]

#[derive(sqlx::FromRow, Debug, PartialEq, Eq)]
struct User {
    id: i32,
    name: String,
    email: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create a connection pool
    let pool = SqlitePool::connect("sqlite::memory:").await?;

    // In SQL, queries can be separated into prepared (parameterized) or
    // unprepared (simple). Prepared queries have their query plan cached,
    // use a binary mode of communication (lower bandwidth and faster decoding),
    // and utilize parameters to avoid SQL injection. Unprepared queries are
    // simple and intended only for use where a prepared statement will not
    // work, such as various database commands (e.g., PRAGMA or SET or
    // BEGIN). In SQLx, a `&str` is treated as an unprepared query,
    // and a `Query` or `QueryAs` struct is treated as a prepared query.

    // Create the users table using a prepared, cached query
    sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            email TEXT NOT NULL UNIQUE
        )",
    )
    .execute(&pool)
    .await?;

    // Insert a new user
    sqlx::query("INSERT INTO users (name, email) VALUES (?, ?)")
        .bind("John Doe")
        .bind("john@example.com")
        .execute(&pool)
        .await?;

    // Retrieve all users
    let users: Vec<User> =
        sqlx::query_as::<_, User>("SELECT id, name, email FROM users")
            .fetch_all(&pool)
            .await?;

    // Print the retrieved users
    for user in users {
        println!(
            "ID: {}, Name: {}, Email: {}",
            user.id, user.name, user.email
        );
    }

    Ok(())
}
// ANCHOR_END: example

#[test]
fn require_external_svc() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
