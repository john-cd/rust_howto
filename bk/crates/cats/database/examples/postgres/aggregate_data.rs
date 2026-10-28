#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Group PostgreSQL rows by artist nationality and count each group.

use anyhow::Context as _;
use postgres::Client;
use postgres::NoTls;

/// A nationality and the number of artists in its group.
#[derive(Debug, PartialEq, Eq)]
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
/// Returns an error if `DATABASE_URL` is missing, PostgreSQL cannot be reached,
/// or the query or its result rows are invalid.
fn main() -> anyhow::Result<()> {
    let database_url = std::env::var("DATABASE_URL")
        .context("set DATABASE_URL to connect to PostgreSQL")?;
    let mut client = Client::connect(&database_url, NoTls)
        .context("connect to PostgreSQL using DATABASE_URL")?;

    for nation in artist_counts_by_nationality(&mut client)? {
        println!("{}: {}", nation.nationality, nation.artist_count);
    }

    Ok(())
}

/// Returns artist counts grouped by nationality, including missing values.
///
/// # Errors
///
/// Returns an error if PostgreSQL rejects the query or a result column has an
/// unexpected type.
fn artist_counts_by_nationality(
    client: &mut Client,
) -> anyhow::Result<Vec<Nation>> {
    let query = "SELECT COALESCE(nationality, 'Unknown') AS nationality, \
                 COUNT(*) AS artist_count \
                 FROM artists \
                 GROUP BY COALESCE(nationality, 'Unknown') \
                 ORDER BY artist_count DESC, nationality";

    client
        .query(query, &[])
        .context("count artists by nationality")?
        .iter()
        .map(|row| {
            Ok(Nation {
                nationality: row.try_get("nationality")?,
                artist_count: row.try_get("artist_count")?,
            })
        })
        .collect()
}
// ANCHOR_END: example

/// Runs the PostgreSQL aggregation example.
///
/// # Errors
///
/// Returns an error if `DATABASE_URL` is missing or PostgreSQL query execution
/// fails.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Aggregates the prepared artists table in a PostgreSQL service.
    ///
    /// Run with `DATABASE_URL` set to a reachable PostgreSQL database.
    #[test]
    #[ignore = "needs Postgres"]
    fn aggregates_artist_counts_by_nationality_on_postgres()
    -> anyhow::Result<()> {
        let database_url = std::env::var("DATABASE_URL")
            .context("set DATABASE_URL to connect to PostgreSQL")?;
        let mut client = Client::connect(&database_url, NoTls)
            .context("connect to PostgreSQL using DATABASE_URL")?;
        client.batch_execute("CREATE TEMP TABLE artists (nationality TEXT)")?;
        client.execute(
            "INSERT INTO artists (nationality) VALUES ($1), ($2), ($3), ($4)",
            &[
                &Some("Known"),
                &Some("Known"),
                &Some("Other"),
                &None::<&str>,
            ],
        )?;

        assert_eq!(
            artist_counts_by_nationality(&mut client)?,
            [
                Nation {
                    nationality: "Known".to_owned(),
                    artist_count: 2,
                },
                Nation {
                    nationality: "Other".to_owned(),
                    artist_count: 1,
                },
                Nation {
                    nationality: "Unknown".to_owned(),
                    artist_count: 1,
                },
            ],
            "nationalities are grouped and ties are ordered by name"
        );
        Ok(())
    }

    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
