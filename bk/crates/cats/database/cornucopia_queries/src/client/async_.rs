//! \ Async\ client\ interfaces\ and\ implementations\ for\ generated\ database\ queries\.
//!
//! TODO Reviewed ??-??-2026.

pub use generic_client::GenericClient;

/// This module provides the `deadpool` implementation used by its parent.
#[cfg(feature = "deadpool")]
mod deadpool;

/// This module provides the `generic_client` implementation used by its parent.
mod generic_client;
use tokio_postgres::Error;
use tokio_postgres::Row;
use tokio_postgres::RowStream;
use tokio_postgres::Statement;
use tokio_postgres::types::BorrowToSql;
use tokio_postgres::types::ToSql;

/// This trait allows you to bind parameters to a query using a single
/// struct, rather than passing each bind parameter as a function parameter.
pub trait Params<'c, 'a, 's, P, O, C> {
    /// Binds the borrowed parameter record and client to the generated query output type.
    fn params(&'s self, client: &'c C, params: &'a P) -> O;
}

/// Executes a query that must return exactly one row.
///
/// # Errors
///
/// Returns a database error if preparing the statement or executing it does not return exactly one
/// row.
pub async fn one<C: GenericClient>(
    client: &C,
    query: &str,
    params: &[&(dyn ToSql + Sync)],
    cached: Option<&Statement>,
) -> Result<Row, Error> {
    if let Some(cached) = cached {
        client.query_one(cached, params).await
    } else if C::stmt_cache() {
        let cached = client.prepare(query).await?;
        client.query_one(&cached, params).await
    } else {
        client.query_one(query, params).await
    }
}

/// Executes a query that may return zero or one row.
///
/// # Errors
///
/// Returns a database error if preparing the statement or executing it fails or returns multiple
/// rows.
pub async fn opt<C: GenericClient>(
    client: &C,
    query: &str,
    params: &[&(dyn ToSql + Sync)],
    cached: Option<&Statement>,
) -> Result<Option<Row>, Error> {
    if let Some(cached) = cached {
        client.query_opt(cached, params).await
    } else if C::stmt_cache() {
        let cached = client.prepare(query).await?;
        client.query_opt(&cached, params).await
    } else {
        client.query_opt(query, params).await
    }
}

/// Executes a query and returns its rows as a stream.
///
/// # Errors
///
/// Returns a database error if preparing or starting the row query fails.
pub async fn raw<C: GenericClient, P, I>(
    client: &C,
    query: &str,
    params: I,
    cached: Option<&Statement>,
) -> Result<RowStream, Error>
where
    P: BorrowToSql,
    I: IntoIterator<Item = P> + Sync + Send,
    I::IntoIter: ExactSizeIterator,
{
    if let Some(cached) = cached {
        client.query_raw(cached, params).await
    } else if C::stmt_cache() {
        let cached = client.prepare(query).await?;
        client.query_raw(&cached, params).await
    } else {
        client.query_raw(query, params).await
    }
}
