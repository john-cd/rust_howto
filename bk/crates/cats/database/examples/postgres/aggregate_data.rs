#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Group PostgreSQL rows by artist nationality and count each group.

use anyhow::Context as _;
use postgres::Client;
use postgres::NoTls;

/// A nationality and the number of artists in its group.
struct Nation {
    /// Nationality name, or `Unknown` for rows without one.
    nationality: String,
    /// Number of artist rows in the group.
    artist_count: i64,
}

/// Queries an existing `artists` table and prints each nationality count.
///
/// # Errors
///
/// Returns an error if `PG_URL` is missing, PostgreSQL cannot be reached, or
/// the query or its result rows are invalid.
fn main() -> anyhow::Result<()> {
    let url = std::env::var("PG_URL")
        .context("set PG_URL to connect to the PostgreSQL database")?;
    let mut client = Client::connect(&url, NoTls)
        .context("connect to PostgreSQL with PG_URL")?;

    let query = "SELECT COALESCE(nationality, 'Unknown') AS nationality, \
                 COUNT(*) AS artist_count \
                 FROM artists \
                 GROUP BY COALESCE(nationality, 'Unknown') \
                 ORDER BY artist_count DESC, nationality";

    for row in client
        .query(query, &[])
        .context("count artists by nationality")?
    {
        let nation = Nation {
            nationality: row.try_get("nationality")?,
            artist_count: row.try_get("artist_count")?,
        };
        println!("{}: {}", nation.nationality, nation.artist_count);
    }

    Ok(())
}
// ANCHOR_END: example

/// Runs the PostgreSQL aggregation example.
///
/// # Errors
///
/// Returns an error if `PG_URL` is missing or PostgreSQL query execution fails.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Aggregates the prepared artists table in a PostgreSQL service.
    ///
    /// Run with `PG_URL` set to a database that contains an `artists` table.
    #[test]
    #[ignore = "requires PostgreSQL and an existing artists table"]
    fn aggregates_artist_counts_by_nationality() -> anyhow::Result<()> {
        main()
    }
}
