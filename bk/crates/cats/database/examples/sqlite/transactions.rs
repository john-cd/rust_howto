// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This example demonstrates successful and rolled-back SQLite transactions.

use anyhow::Result;
use rusqlite::Connection;
use rusqlite::Error;
use rusqlite::ErrorCode;

/// Creates the tables required by the transaction example.
fn initialize_schema(connection: &Connection) -> rusqlite::Result<()> {
    connection.execute_batch(
        "CREATE TABLE cat_colors (
             id INTEGER PRIMARY KEY,
             name TEXT NOT NULL UNIQUE
         );
         CREATE TABLE cats (
             id INTEGER PRIMARY KEY,
             name TEXT NOT NULL,
             color_id INTEGER NOT NULL REFERENCES cat_colors(id)
         );",
    )
}

/// Demonstrates a committed transaction followed by a rolled-back transaction.
///
/// # Errors
///
/// Returns errors from opening the in-memory database, creating its schema, or
/// running either transaction when it fails for a reason other than the
/// expected unique-constraint violation.
fn main() -> Result<()> {
    let mut connection = Connection::open_in_memory()?;
    initialize_schema(&connection)?;

    successful_tx(&mut connection)?;
    println!(
        "Successful transaction committed: {:?}",
        color_names(&connection)?
    );

    match rolled_back_tx(&mut connection) {
        Err(Error::SqliteFailure(code, _))
            if code.code == ErrorCode::ConstraintViolation
                && code.extended_code
                    == rusqlite::ffi::SQLITE_CONSTRAINT_UNIQUE =>
        {
            println!(
                "Duplicate color rejected; the transaction was rolled back."
            );
        }
        Err(error) => return Err(error.into()),
        Ok(()) => {
            anyhow::bail!("duplicate color insert unexpectedly succeeded")
        }
    }

    Ok(())
}

/// Inserts two colors and commits them in a single transaction.
///
/// # Errors
///
/// Returns a SQLite error if any statement or the commit fails.
fn successful_tx(connection: &mut Connection) -> rusqlite::Result<()> {
    let transaction = connection.transaction()?;
    transaction.execute("DELETE FROM cats", ())?;
    transaction.execute("DELETE FROM cat_colors", ())?;
    transaction
        .execute("INSERT INTO cat_colors (name) VALUES (?1)", [&"lavender"])?;
    transaction
        .execute("INSERT INTO cat_colors (name) VALUES (?1)", [&"blue"])?;
    transaction.commit()
}

/// Attempts duplicate inserts so SQLite returns an error and the transaction
/// is rolled back when it is dropped.
///
/// # Errors
///
/// Returns the unique-constraint error for the duplicate color, or another
/// SQLite error if an earlier statement fails.
fn rolled_back_tx(connection: &mut Connection) -> rusqlite::Result<()> {
    let transaction = connection.transaction()?;
    transaction.execute("DELETE FROM cats", ())?;
    transaction.execute("DELETE FROM cat_colors", ())?;
    transaction
        .execute("INSERT INTO cat_colors (name) VALUES (?1)", [&"lavender"])?;
    transaction
        .execute("INSERT INTO cat_colors (name) VALUES (?1)", [&"blue"])?;
    transaction
        .execute("INSERT INTO cat_colors (name) VALUES (?1)", [&"lavender"])?;
    transaction.commit()
}

/// Reads color names in insertion order so callers can verify transaction
/// state.
///
/// # Errors
///
/// Returns a SQLite error if the query cannot be prepared or read.
fn color_names(connection: &Connection) -> rusqlite::Result<Vec<String>> {
    let mut statement =
        connection.prepare("SELECT name FROM cat_colors ORDER BY id")?;
    let rows = statement.query_map([], |row| row.get(0))?;
    rows.collect()
}
// ANCHOR_END: example

/// Runs the transaction example.
///
/// # Errors
///
/// Returns any unexpected database error from the example.
#[cfg(target_os = "linux")]
pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use rusqlite::Connection;
    use rusqlite::Error;
    use rusqlite::ErrorCode;

    use super::color_names;
    use super::initialize_schema;
    use super::main;
    use super::rolled_back_tx;
    use super::successful_tx;

    /// Creates an isolated in-memory database with the example schema.
    fn test_connection() -> rusqlite::Result<Connection> {
        let connection = Connection::open_in_memory()?;
        initialize_schema(&connection)?;
        Ok(connection)
    }

    /// Verifies that the successful transaction commits both inserted colors.
    #[test]
    fn successful_transaction_commits_rows() -> Result<()> {
        let mut connection = test_connection()?;

        successful_tx(&mut connection)?;

        assert_eq!(
            color_names(&connection)?,
            vec![String::from("lavender"), String::from("blue")]
        );
        Ok(())
    }

    /// Verifies that a unique-constraint error rolls back the transaction.
    #[test]
    fn duplicate_color_error_rolls_back_transaction() -> Result<()> {
        let mut connection = test_connection()?;
        successful_tx(&mut connection)?;

        let error = match rolled_back_tx(&mut connection) {
            Ok(()) => {
                anyhow::bail!("duplicate color insert unexpectedly succeeded")
            }
            Err(error) => error,
        };

        assert!(matches!(
            error,
            Error::SqliteFailure(code, _)
                if code.code == ErrorCode::ConstraintViolation
                    && code.extended_code == rusqlite::ffi::SQLITE_CONSTRAINT_UNIQUE
        ));
        assert_eq!(
            color_names(&connection)?,
            vec![String::from("lavender"), String::from("blue")]
        );
        Ok(())
    }

    #[test]
    fn example_handles_expected_duplicate_error() -> Result<()> {
        main()
    }
}
